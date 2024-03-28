use std::cell::RefCell;
use std::collections::HashMap;

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
use crate::bindings::sputnik::registry::api::{Asset, SpotPair};
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
    registry_api: Box<dyn RegistryApi>,
    orders: HashMap<u64, OrderAndStatus>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        balances: HashMap::new(),
        id: None,
        registry_api: Box::new(RegistryApiProd),
        orders: HashMap::new(),
    });
}
fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

#[automock]
trait RegistryApi {
    fn get_registry(&self) -> stub_registry::Api;
    fn get_assets(&self) -> HashMap<u64, Asset>;
    fn get_spot_pairs(&self) -> HashMap<u64, SpotPair>;
    fn get_matching_engine(&self, spot_pair_id: u64) -> stub_matching_engine::Api;
    fn place_order(
        &self,
        spot_pair: u64,
        order: stub_matching_engine::Order,
    ) -> Result<stub_matching_engine::OrderStatus, stub_matching_engine::Error>;
}

pub struct RegistryApiProd;

impl RegistryApi for RegistryApiProd {
    fn get_registry(&self) -> stub_registry::Api {
        let template_id =
            std::env::var("REGISTRY_TEMPLATE_ID").expect("REGISTRY_TEMPLATE_ID not set");
        let uri = Uri {
            value: format!("worker://{template_id}/{}", "registry"),
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

    fn get_spot_pairs(&self) -> HashMap<u64, SpotPair> {
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
        let uri = Uri {
            value: format!("worker://{template_id}/{spot_pair_id}"),
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
    let spot_pairs = state.registry_api.get_spot_pairs();
    match state.balances.get(&asset_id) {
        Some(balance) => {
            *balance
                - state
                    .orders
                    .iter()
                    .filter_map(|(_, order_and_status)| match order_and_status {
                        OrderAndStatus { order, status } => {
                            let spot_pair = spot_pairs.get(&order.spot_pair);
                            let remaining_size = status.original_size
                                - status.fills.iter().map(|fill| fill.size).sum::<u64>();
                            match order.side {
                                Buy => {
                                    if spot_pair.unwrap().denominator.id == asset_id {
                                        Some(
                                            remaining_size * order.price
                                                / decimal_power(
                                                    spot_pair.unwrap().numerator.decimals,
                                                ),
                                        )
                                    } else {
                                        None
                                    }
                                }
                                Sell => {
                                    if spot_pair.unwrap().numerator.id == asset_id {
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
            let spot_pairs = state.registry_api.get_spot_pairs();
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
            let assets = state.registry_api.get_assets();
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
            let spot_pairs = state.registry_api.get_spot_pairs();
            match spot_pairs.get(&order.spot_pair) {
                Some(spot_pair) => {
                    let (required_asset, required_balance) = match order.side {
                        Buy => (
                            spot_pair.denominator.clone(),
                            order.size * order.price / spot_pair.numerator.decimals as u64,
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
                            trader: state.id.unwrap(),
                        };
                        match state
                            .registry_api
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
            let assets = state.registry_api.get_assets();
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
            let assets = state.registry_api.get_assets();
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

    use crate::{Component, Guest, MockRegistryApi, with_state};
    use crate::bindings::exports::sputnik::accountant::api::AssetBalance;
    use crate::bindings::sputnik::registry::api::{Asset, SpotPair};

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

    fn setup_mock_registry() {
        let mut mock_registry_api = MockRegistryApi::new();
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
                SpotPair {
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
        with_state(|state| state.registry_api = Box::new(mock_registry_api));
    }

    fn perform_deposit() -> AssetBalance {
        <Component as Guest>::deposit(1, 100).expect("successful deposit")
    }

    fn perform_withdrawal() -> AssetBalance {
        <Component as Guest>::withdraw(1, 50).expect("successful withdrawal")
    }
    #[test]
    fn test_deposit() {
        setup_mock_registry();

        let asset_balance = perform_deposit();
        assert_eq!(
            asset_balance,
            AssetBalance {
                asset: Asset {
                    id: 1,
                    name: "BTC".to_string(),
                    decimals: 8
                },
                balance: 100,
                available_balance: 100
            }
        );
    }

    #[test]
    fn test_withdrawal() {
        setup_mock_registry();

        perform_deposit();
        let asset_balance = perform_withdrawal();
        assert_eq!(
            asset_balance,
            AssetBalance {
                asset: Asset {
                    id: 1,
                    name: "BTC".to_string(),
                    decimals: 8
                },
                balance: 50,
                available_balance: 50
            }
        );
    }

    #[test]
    fn test_get_balances() {
        setup_mock_registry();

        perform_deposit();
        let balances = <Component as Guest>::get_balances();
        assert_eq!(
            balances,
            vec![AssetBalance {
                asset: Asset {
                    id: 1,
                    name: "BTC".to_string(),
                    decimals: 8
                },
                balance: 100,
                available_balance: 100
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
                balance: 50,
                available_balance: 50
            }]
        );
    }
}
