use std::cell::RefCell;
use std::collections::HashMap;
use std::env;

use mockall::automock;

use crate::bindings::exports::sputnik::accountant::api::{
    AssetBalance, Error, Fill, Guest, Order, OrderStatus,
};
use crate::bindings::exports::sputnik::accountant::api::Error::{
    AlreadyInitialized, InsufficientFunds, InvalidAsset, InvalidSpotPair, MatchingEngineError,
};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::sputnik::matching_engine;
use crate::bindings::sputnik::matching_engine::api::Side::{Buy, Sell};
use crate::bindings::sputnik::matching_engine_stub::stub_matching_engine;
use crate::bindings::sputnik::registry::api::{Asset, HydratedSpotPair};
use crate::bindings::sputnik::registry_stub::stub_registry;

mod bindings;

struct Component;

#[derive(Clone)]
struct OrderAndStatus {
    order: Order,
    status: stub_matching_engine::OrderStatus,
}
struct State {
    id: Option<u64>,
    balances: HashMap<u64, u64>,
    external_service_api: Box<dyn ExternalServiceApi>,
    orders: HashMap<u64, OrderAndStatus>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        balances: HashMap::new(),
        id: None,
        external_service_api: Box::new(ExternalServiceApiProd),
        orders: HashMap::new(),
    });
}
fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

#[automock]
trait ExternalServiceApi {
    fn get_registry(&self) -> stub_registry::Api;
    fn get_assets(&self) -> HashMap<u64, Asset>;
    fn get_spot_pairs(&self) -> HashMap<u64, HydratedSpotPair>;
    fn get_matching_engine(&self, spot_pair_id: u64) -> stub_matching_engine::Api;
    fn place_order(
        &self,
        spot_pair: u64,
        order: stub_matching_engine::Order,
    ) -> Result<stub_matching_engine::OrderStatus, stub_matching_engine::Error>;
}

pub struct ExternalServiceApiProd;

impl ExternalServiceApi for ExternalServiceApiProd {
    fn get_registry(&self) -> stub_registry::Api {
        let template_id = env::var("REGISTRY_TEMPLATE_ID").expect("REGISTRY_TEMPLATE_ID not set");
        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT NOT SET");
        let uri = Uri {
            value: format!("worker://{template_id}/{environment}"),
        };

        stub_registry::Api::new(&uri)
    }

    fn get_assets(&self) -> HashMap<u64, Asset> {
        HashMap::from_iter(
            self.get_registry()
                .get_assets()
                .iter()
                .map(|asset| (asset.id, asset.clone())),
        )
    }

    fn get_spot_pairs(&self) -> HashMap<u64, HydratedSpotPair> {
        HashMap::from_iter(
            self.get_registry()
                .get_spot_pairs()
                .iter()
                .map(|spot_pair| (spot_pair.id, spot_pair.clone())),
        )
    }

    fn get_matching_engine(&self, spot_pair_id: u64) -> stub_matching_engine::Api {
        let template_id = std::env::var("MATCHING_ENGINE_TEMPLATE_ID")
            .expect("MATCHING_ENGINE_TEMPLATE_ID not set");
        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT NOT SET");
        let uri = Uri {
            value: format!("worker://{template_id}/{environment}-{spot_pair_id}"),
        };

        stub_matching_engine::Api::new(&uri)
    }

    fn place_order(
        &self,
        spot_pair: u64,
        order: stub_matching_engine::Order,
    ) -> Result<stub_matching_engine::OrderStatus, stub_matching_engine::Error> {
        self.get_matching_engine(spot_pair).place_order(order)
    }
}

const ZERO: u64 = 0u64;

fn available_balance(state: &mut State, asset_id: u64) -> u64 {
    let spot_pairs = state.external_service_api.get_spot_pairs();
    match state.balances.get(&asset_id) {
        Some(balance) => {
            *balance
                - state
                    .orders
                    .iter()
                    .filter_map(|(_, order_and_status)| match order_and_status {
                        OrderAndStatus { order, status } => {
                            let spot_pair = spot_pairs
                                .get(&order.spot_pair)
                                .expect("spot pair map should have pair");
                            let remaining_size = status.original_size
                                - status.fills.iter().map(|fill| fill.size).sum::<u64>();
                            match order.side {
                                Buy => {
                                    if spot_pair.denominator.id == asset_id {
                                        Some(
                                            remaining_size * order.price
                                                / decimal_power(spot_pair.numerator.decimals),
                                        )
                                    } else {
                                        None
                                    }
                                }
                                Sell => {
                                    if spot_pair.numerator.id == asset_id {
                                        Some(remaining_size)
                                    } else {
                                        None
                                    }
                                }
                            }
                        }
                    })
                    .sum::<u64>()
        }
        None => ZERO,
    }
}

const TEN: u64 = 10;

fn decimal_power(decimals: u8) -> u64 {
    TEN.pow(decimals as u32)
}

fn decrease_balance(state: &mut State, asset_id: u64, qty: u64) -> u64 {
    let balance = state.balances.entry(asset_id).or_insert_with(|| ZERO);
    *balance -= qty;
    balance.clone()
}

fn increase_balance(state: &mut State, asset_id: u64, qty: u64) -> u64 {
    let balance = state.balances.entry(asset_id).or_insert_with(|| ZERO);
    *balance += qty;
    balance.clone()
}

fn process_fill(state: &mut State, is_taker: bool, fill: &matching_engine::api::Fill) {
    let order_id = match is_taker {
        true => fill.taker_order_id,
        false => fill.maker_order_id,
    };
    match state.orders.get(&order_id) {
        Some(OrderAndStatus {
            order,
            status: _status,
        }) => {
            let spot_pairs = state.external_service_api.get_spot_pairs();
            let spot_pair = match spot_pairs.get(&order.spot_pair) {
                Some(spot_pair) => spot_pair,
                None => panic!("No spot pair found {}", order.spot_pair),
            };
            let numerator_change = fill.size;
            let denominator_change =
                fill.size * fill.price / decimal_power(spot_pair.numerator.decimals);
            match order.side {
                Buy => {
                    decrease_balance(state, spot_pair.denominator.id, denominator_change);
                    increase_balance(state, spot_pair.numerator.id, numerator_change);
                }
                Sell => {
                    increase_balance(state, spot_pair.denominator.id, denominator_change);
                    decrease_balance(state, spot_pair.numerator.id, numerator_change);
                }
            }
        }
        None => panic!("Order not found: {}", order_id),
    }
}

impl Guest for Component {
    fn initialize(id: u64) -> Result<u64, Error> {
        with_state(|state| match state.id {
            Some(existing_id) => Err(AlreadyInitialized(existing_id)),
            None => {
                state.id = Some(id);
                Ok(id)
            }
        })
    }

    fn get_balances() -> Vec<AssetBalance> {
        with_state(|state| {
            let assets = state.external_service_api.get_assets();
            state
                .balances
                .iter()
                .filter_map(|(asset_id, balance)| match assets.get(asset_id) {
                    Some(asset) => Some((asset.clone(), *balance)),
                    None => None,
                })
                .collect::<Vec<(Asset, u64)>>()
                .iter()
                .map(|(asset, balance)| AssetBalance {
                    asset: asset.clone(),
                    balance: *balance,
                    available_balance: available_balance(state, asset.id.clone()),
                })
                .collect()
        })
    }

    fn place_order(order: Order) -> Result<OrderStatus, Error> {
        with_state(|state| {
            let spot_pairs = state.external_service_api.get_spot_pairs();
            match spot_pairs.get(&order.spot_pair) {
                Some(spot_pair) => {
                    let (required_asset, required_balance) = match order.side {
                        Buy => (
                            spot_pair.denominator.clone(),
                            order.size * order.price / decimal_power(spot_pair.numerator.decimals),
                        ),
                        Sell => (spot_pair.numerator.clone(), order.size),
                    };
                    let available_balance = available_balance(state, required_asset.id);
                    if available_balance < required_balance {
                        Err(InsufficientFunds(available_balance))
                    } else {
                        let matching_engine_order = stub_matching_engine::Order {
                            id: order.id,
                            timestamp: order.timestamp,
                            side: order.side,
                            price: order.price,
                            size: order.size,
                            trader: state
                                .id
                                .expect("accountant should be initialized with trader id"),
                        };
                        match state
                            .external_service_api
                            .place_order(order.spot_pair, matching_engine_order)
                        {
                            Err(matching_engine_error) => {
                                Err(MatchingEngineError(matching_engine_error))
                            }
                            Ok(order_status) => {
                                state.orders.insert(
                                    order.id,
                                    OrderAndStatus {
                                        order,
                                        status: order_status.clone(),
                                    },
                                );
                                order_status
                                    .fills
                                    .iter()
                                    .for_each(|fill| process_fill(state, true, fill));
                                Ok(OrderStatus { id: order.id })
                            }
                        }
                    }
                }
                None => Err(InvalidSpotPair(order.spot_pair)),
            }
        })
    }

    fn deposit(asset_id: u64, amount: u64) -> Result<AssetBalance, Error> {
        with_state(|state| {
            let assets = state.external_service_api.get_assets();
            let balance = increase_balance(state, asset_id, amount);
            match assets.get(&asset_id) {
                Some(asset) => Ok(AssetBalance {
                    asset: asset.clone(),
                    balance,
                    available_balance: available_balance(state, asset_id),
                }),
                None => Err(InvalidAsset(asset_id)),
            }
        })
    }

    fn withdraw(asset_id: u64, amount: u64) -> Result<AssetBalance, Error> {
        with_state(|state| {
            let assets = state.external_service_api.get_assets();
            let available = available_balance(state, asset_id);
            if available <= amount {
                Err(InsufficientFunds(available))
            } else {
                let new_balance = decrease_balance(state, asset_id, amount);
                match assets.get(&asset_id) {
                    Some(asset) => Ok(AssetBalance {
                        asset: asset.clone(),
                        balance: new_balance,
                        available_balance: available_balance(state, asset_id),
                    }),
                    None => Err(InvalidAsset(asset_id)),
                }
            }
        })
    }

    fn process_maker_fill(fill: Fill) {
        with_state(|state| {
            process_fill(
                state,
                false,
                &matching_engine::api::Fill {
                    price: fill.price,
                    size: fill.size,
                    taker_order_id: fill.taker_order_id,
                    maker_order_id: fill.maker_order_id,
                    timestamp: fill.timestamp,
                },
            );
            match state.orders.get_mut(&fill.maker_order_id) {
                None => panic!("No order {}", fill.maker_order_id),
                Some(OrderAndStatus {
                    order: _order,
                    status,
                }) => status.fills.push(fill),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use assert_unordered::assert_eq_unordered;

    use crate::{Component, Guest, MockExternalServiceApi, with_state};
    use crate::bindings::exports::sputnik::accountant::api::{AssetBalance, Order};
    use crate::bindings::sputnik::matching_engine::api::Fill;
    use crate::bindings::sputnik::matching_engine::api::Side::{Buy, Sell};
    use crate::bindings::sputnik::matching_engine::api::Status::{Filled, Open, PartialFilled};
    use crate::bindings::sputnik::matching_engine_stub::stub_matching_engine::OrderStatus;
    use crate::bindings::sputnik::registry::api::{Asset, HydratedSpotPair};

    impl PartialEq for AssetBalance {
        fn eq(&self, other: &Self) -> bool {
            self.balance == other.balance
                && self.available_balance == other.available_balance
                && self.asset == other.asset
        }
    }

    impl PartialEq for Asset {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id && self.name == other.name && self.decimals == other.decimals
        }
    }

    impl PartialEq for crate::bindings::exports::sputnik::accountant::api::OrderStatus {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    fn init() {
        <Component as Guest>::initialize(1).expect("successful initialization");
    }

    fn setup_mock_registry(fills: f32) {
        let mut mock_registry_api = MockExternalServiceApi::new();
        mock_registry_api
            .expect_get_assets()
            .return_const(HashMap::from_iter(vec![
                (
                    1,
                    Asset {
                        id: 1,
                        name: "BTC".to_string(),
                        decimals: 8,
                    },
                ),
                (
                    2,
                    Asset {
                        id: 2,
                        name: "USD".to_string(),
                        decimals: 2,
                    },
                ),
            ]));
        mock_registry_api
            .expect_get_spot_pairs()
            .return_const(HashMap::from_iter(vec![(
                1,
                HydratedSpotPair {
                    id: 1,
                    name: "BTCUSD".to_string(),
                    numerator: Asset {
                        id: 1,
                        name: "BTC".to_string(),
                        decimals: 8,
                    },
                    denominator: Asset {
                        id: 2,
                        name: "USD".to_string(),
                        decimals: 2,
                    },
                },
            )]));
        mock_registry_api
            .expect_place_order()
            .returning(move |_, order| {
                if fills == 0f32 {
                    Ok(OrderStatus {
                        id: order.id,
                        fills: vec![],
                        status: Open,
                        original_size: order.size,
                    })
                } else if fills < 1f32 {
                    Ok(OrderStatus {
                        id: order.id,
                        fills: vec![Fill {
                            price: order.price,
                            size: order.size / (1.0 / fills) as u64,
                            taker_order_id: order.id,
                            maker_order_id: 0,
                            timestamp: 0,
                        }],
                        status: PartialFilled,
                        original_size: order.size,
                    })
                } else {
                    Ok(OrderStatus {
                        id: order.id,
                        fills: vec![Fill {
                            price: order.price,
                            size: order.size,
                            taker_order_id: order.id,
                            maker_order_id: 0,
                            timestamp: 0,
                        }],
                        status: Filled,
                        original_size: order.size,
                    })
                }
            });
        with_state(|state| state.external_service_api = Box::new(mock_registry_api));
    }

    fn perform_btc_deposit() -> AssetBalance {
        <Component as Guest>::deposit(1, 100000000).expect("successful deposit")
    }

    fn perform_usd_deposit() -> AssetBalance {
        <Component as Guest>::deposit(2, 6000000).expect("successful deposit")
    }

    fn perform_withdrawal() -> AssetBalance {
        <Component as Guest>::withdraw(1, 50000000).expect("successful withdrawal")
    }
    #[test]
    fn test_deposit() {
        setup_mock_registry(0f32);

        let asset_balance = perform_btc_deposit();
        assert_eq!(
            asset_balance,
            AssetBalance {
                asset: Asset {
                    id: 1,
                    name: "BTC".to_string(),
                    decimals: 8
                },
                balance: 100000000,
                available_balance: 100000000,
            }
        );
    }

    #[test]
    fn test_withdrawal() {
        setup_mock_registry(0f32);
        perform_btc_deposit();
        let asset_balance = perform_withdrawal();
        assert_eq!(
            asset_balance,
            AssetBalance {
                asset: Asset {
                    id: 1,
                    name: "BTC".to_string(),
                    decimals: 8
                },
                balance: 50000000,
                available_balance: 50000000,
            }
        );
    }

    #[test]
    fn test_get_balances() {
        setup_mock_registry(0f32);

        perform_btc_deposit();
        let balances = <Component as Guest>::get_balances();
        assert_eq!(
            balances,
            vec![AssetBalance {
                asset: Asset {
                    id: 1,
                    name: "BTC".to_string(),
                    decimals: 8
                },
                balance: 100000000,
                available_balance: 100000000,
            }]
        );
        perform_withdrawal();
        let balances = <Component as Guest>::get_balances();
        assert_eq!(
            balances,
            vec![AssetBalance {
                asset: Asset {
                    id: 1,
                    name: "BTC".to_string(),
                    decimals: 8
                },
                balance: 50000000,
                available_balance: 50000000,
            }]
        );
    }

    #[test]
    fn test_place_sell_order_no_fills() {
        init();
        setup_mock_registry(0f32);
        perform_btc_deposit();
        let status = <Component as Guest>::place_order(Order {
            id: 1,
            spot_pair: 1,
            timestamp: 0,
            side: Sell,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 1 }
        );
        let balances = <Component as Guest>::get_balances();
        assert_eq!(
            balances,
            vec![AssetBalance {
                asset: Asset {
                    id: 1,
                    name: "BTC".to_string(),
                    decimals: 8
                },
                balance: 100000000,
                available_balance: 75000000,
            }]
        );
    }

    #[test]
    fn test_place_buy_order_no_fills() {
        init();
        setup_mock_registry(0f32);
        perform_usd_deposit();
        let status = <Component as Guest>::place_order(Order {
            id: 1,
            spot_pair: 1,
            timestamp: 0,
            side: Buy,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 1 }
        );
        let balances = <Component as Guest>::get_balances();
        assert_eq!(
            balances,
            vec![AssetBalance {
                asset: Asset {
                    id: 2,
                    name: "USD".to_string(),
                    decimals: 2
                },
                balance: 6000000,
                available_balance: 4500000,
            }]
        );
    }

    #[test]
    fn test_place_sell_order_partial_fill() {
        init();
        setup_mock_registry(0.50f32);
        perform_btc_deposit();
        let status = <Component as Guest>::place_order(Order {
            id: 1,
            spot_pair: 1,
            timestamp: 0,
            side: Sell,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 1 }
        );
        let balances = <Component as Guest>::get_balances();
        assert_eq_unordered!(
            &balances,
            &vec![
                AssetBalance {
                    asset: Asset {
                        id: 1,
                        name: "BTC".to_string(),
                        decimals: 8
                    },
                    balance: 87500000,
                    available_balance: 75000000
                },
                AssetBalance {
                    asset: Asset {
                        id: 2,
                        name: "USD".to_string(),
                        decimals: 2
                    },
                    balance: 750000,
                    available_balance: 750000
                }
            ]
        );
    }

    #[test]
    fn test_place_buy_order_partial_fill() {
        init();
        setup_mock_registry(0.50f32);
        perform_usd_deposit();
        let status = <Component as Guest>::place_order(Order {
            id: 1,
            spot_pair: 1,
            timestamp: 0,
            side: Buy,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 1 }
        );
        let balances = <Component as Guest>::get_balances();
        assert_eq_unordered!(
            &balances,
            &vec![
                AssetBalance {
                    asset: Asset {
                        id: 1,
                        name: "BTC".to_string(),
                        decimals: 8,
                    },
                    balance: 12500000,
                    available_balance: 12500000,
                },
                AssetBalance {
                    asset: Asset {
                        id: 2,
                        name: "USD".to_string(),
                        decimals: 2,
                    },
                    balance: 5250000,
                    available_balance: 4500000,
                },
            ],
        );
    }

    #[test]
    fn test_place_sell_order_complete_fill() {
        init();
        setup_mock_registry(1f32);
        perform_btc_deposit();
        let status = <Component as Guest>::place_order(Order {
            id: 1,
            spot_pair: 1,
            timestamp: 0,
            side: Sell,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 1 }
        );
        let balances = <Component as Guest>::get_balances();
        assert_eq_unordered!(
            &balances,
            &vec![
                AssetBalance {
                    asset: Asset {
                        id: 1,
                        name: "BTC".to_string(),
                        decimals: 8,
                    },
                    balance: 75000000,
                    available_balance: 75000000,
                },
                AssetBalance {
                    asset: Asset {
                        id: 2,
                        name: "USD".to_string(),
                        decimals: 2,
                    },
                    balance: 1500000,
                    available_balance: 1500000,
                },
            ],
        );
    }

    #[test]
    fn test_place_buy_order_complete_fill() {
        init();
        setup_mock_registry(1f32);
        perform_usd_deposit();
        let status = <Component as Guest>::place_order(Order {
            id: 1,
            spot_pair: 1,
            timestamp: 0,
            side: Buy,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 1 }
        );
        let balances = <Component as Guest>::get_balances();
        assert_eq_unordered!(
            &balances,
            &vec![
                AssetBalance {
                    asset: Asset {
                        id: 1,
                        name: "BTC".to_string(),
                        decimals: 8,
                    },
                    balance: 25000000,
                    available_balance: 25000000,
                },
                AssetBalance {
                    asset: Asset {
                        id: 2,
                        name: "USD".to_string(),
                        decimals: 2,
                    },
                    balance: 4500000,
                    available_balance: 4500000,
                },
            ],
        );
    }

    #[test]
    fn test_place_multiple_orders() {
        init();
        setup_mock_registry(0f32);
        perform_usd_deposit();
        perform_btc_deposit();
        let status = <Component as Guest>::place_order(Order {
            id: 1,
            spot_pair: 1,
            timestamp: 0,
            side: Buy,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 1 }
        );
        let status = <Component as Guest>::place_order(Order {
            id: 2,
            spot_pair: 1,
            timestamp: 0,
            side: Buy,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 2 }
        );
        let status = <Component as Guest>::place_order(Order {
            id: 3,
            spot_pair: 1,
            timestamp: 0,
            side: Sell,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 3 }
        );
        let status = <Component as Guest>::place_order(Order {
            id: 4,
            spot_pair: 1,
            timestamp: 0,
            side: Sell,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 4 }
        );
        let balances = <Component as Guest>::get_balances();
        assert_eq_unordered!(
            &balances,
            &vec![
                AssetBalance {
                    asset: Asset {
                        id: 1,
                        name: "BTC".to_string(),
                        decimals: 8,
                    },
                    balance: 100000000,
                    available_balance: 50000000,
                },
                AssetBalance {
                    asset: Asset {
                        id: 2,
                        name: "USD".to_string(),
                        decimals: 2,
                    },
                    balance: 6000000,
                    available_balance: 3000000,
                },
            ],
        );
    }

    #[test]
    fn test_place_buy_order_maker_fill() {
        init();
        setup_mock_registry(0f32);
        perform_usd_deposit();
        let status = <Component as Guest>::place_order(Order {
            id: 1,
            spot_pair: 1,
            timestamp: 0,
            side: Buy,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 1 }
        );
        <Component as Guest>::process_maker_fill(Fill {
            price: 5500000,
            size: 10000000,
            taker_order_id: 0,
            maker_order_id: 1,
            timestamp: 0,
        });
        <Component as Guest>::process_maker_fill(Fill {
            price: 5500000,
            size: 2500000,
            taker_order_id: 0,
            maker_order_id: 1,
            timestamp: 0,
        });
        let balances = <Component as Guest>::get_balances();
        assert_eq_unordered!(
            &balances,
            &vec![
                AssetBalance {
                    asset: Asset {
                        id: 1,
                        name: "BTC".to_string(),
                        decimals: 8,
                    },
                    balance: 12500000,
                    available_balance: 12500000,
                },
                AssetBalance {
                    asset: Asset {
                        id: 2,
                        name: "USD".to_string(),
                        decimals: 2,
                    },
                    balance: 5312500,
                    available_balance: 4562500,
                },
            ],
        );
    }

    #[test]
    fn test_place_sell_order_maker_fill() {
        init();
        setup_mock_registry(0f32);
        perform_btc_deposit();
        let status = <Component as Guest>::place_order(Order {
            id: 1,
            spot_pair: 1,
            timestamp: 0,
            side: Sell,
            price: 6000000,
            size: 25000000,
        })
        .expect("success");
        assert_eq!(
            status,
            crate::bindings::exports::sputnik::accountant::api::OrderStatus { id: 1 }
        );
        <Component as Guest>::process_maker_fill(Fill {
            price: 6500000,
            size: 10000000,
            taker_order_id: 0,
            maker_order_id: 1,
            timestamp: 0,
        });
        <Component as Guest>::process_maker_fill(Fill {
            price: 6500000,
            size: 2500000,
            taker_order_id: 0,
            maker_order_id: 1,
            timestamp: 0,
        });
        let balances = <Component as Guest>::get_balances();
        assert_eq_unordered!(
            &balances,
            &vec![
                AssetBalance {
                    asset: Asset {
                        id: 1,
                        name: "BTC".to_string(),
                        decimals: 8
                    },
                    balance: 87500000,
                    available_balance: 75000000
                },
                AssetBalance {
                    asset: Asset {
                        id: 2,
                        name: "USD".to_string(),
                        decimals: 2
                    },
                    balance: 812500,
                    available_balance: 812500
                }
            ]
        );
    }
}
