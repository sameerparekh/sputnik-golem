use std::cell::RefCell;
use std::collections::HashMap;

use mockall::automock;

use crate::bindings::exports::sputnik::accountant::api::{
    AssetBalance, Error, Guest, Order, OrderStatus,
};
use crate::bindings::exports::sputnik::accountant::api::Error::{
    AlreadyInitialized, InsufficientFunds,
};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::sputnik::registry::api::Asset;
use crate::bindings::sputnik::registry_stub::stub_registry;

mod bindings;

struct Component;

struct State {
    id: Option<u64>,
    balances: HashMap<u64, u64>,
    registry_api: Box<dyn RegistryApi>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        balances: HashMap::new(),
        id: None,
        registry_api: Box::new(RegistryApiProd),
    });
}
fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

#[automock]
trait RegistryApi {
    fn get_registry(&self) -> stub_registry::Api;
    fn get_assets(&self) -> Vec<Asset>;
}

pub struct RegistryApiProd;

#[automock]
impl RegistryApi for RegistryApiProd {
    fn get_registry(&self) -> stub_registry::Api {
        let template_id =
            std::env::var("REGISTRY_TEMPLATE_ID").expect("REGISTRY_TEMPLATE_ID not set");
        let uri = Uri {
            value: format!("worker://{template_id}/{}", "registry"),
        };

        stub_registry::Api::new(&uri)
    }

    fn get_assets(&self) -> Vec<Asset> {
        self.get_registry().get_assets()
    }
}

const ZERO: u64 = 0u64;

// TODO: Actually calculate available balance given orders
fn available_balance(state: &mut State, asset_id: u64) -> u64 {
    match state.balances.get(&asset_id) {
        Some(balance) => *balance,
        None => ZERO,
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
                .filter_map(|(asset_id, balance)| {
                    match assets.iter().find(|asset| asset.id == *asset_id) {
                        Some(asset) => Some((asset.clone(), *balance)),
                        None => None,
                    }
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
        todo!()
    }

    fn deposit(asset_id: u64, amount: u64) -> AssetBalance {
        with_state(|state| {
            let assets = state.registry_api.get_assets();
            let balance = state.balances.entry(asset_id).or_insert_with(|| ZERO);
            *balance += amount;
            match assets.iter().find(|asset| asset.id == asset_id) {
                Some(asset) => AssetBalance {
                    asset: asset.clone(),
                    balance: *balance,
                    available_balance: available_balance(state, asset_id),
                },
                None => panic!("No asset_id {}", asset_id),
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
                state
                    .balances
                    .entry(asset_id)
                    .and_modify(|amt| *amt -= amount);
                match assets.iter().find(|asset| asset.id == asset_id) {
                    Some(asset) => Ok(AssetBalance {
                        asset: asset.clone(),
                        balance: state.balances.get(&asset_id).unwrap().clone(),
                        available_balance: available_balance(state, asset_id),
                    }),
                    None => panic!("No asset_id {}", asset_id),
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Component, Guest, MockRegistryApi, with_state};
    use crate::bindings::exports::sputnik::accountant::api::AssetBalance;
    use crate::bindings::sputnik::registry::api::Asset;

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

    #[test]
    fn test_deposit() {
        let mut mock_registry_api = MockRegistryApi::new();
        mock_registry_api
            .expect_get_assets()
            .return_const(vec![Asset {
                id: 1,
                name: "BTC".to_string(),
                decimals: 8,
            }]);
        with_state(|state| state.registry_api = Box::new(mock_registry_api));
        <Component as Guest>::deposit(1, 100);
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
    }
}
