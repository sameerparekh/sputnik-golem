use std::cell::RefCell;
use std::collections::HashMap;

use crate::bindings::exports::sputnik::accountant::api::{
    AssetBalance, Error, Guest, Order, OrderStatus,
};
use crate::bindings::exports::sputnik::accountant::api::Error::{
    AlreadyInitialized, InsufficientFunds,
};
use crate::bindings::golem::rpc::types::RpcError::RemoteInternalError;
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::sputnik::registry_stub::stub_registry;

mod bindings;

struct Component;

struct State {
    id: Option<u64>,
    balances: HashMap<u64, u64>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        balances: HashMap::new(),
        id: None,
    });
}
fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

fn get_registry() -> stub_registry::Api {
    let template_id = std::env::var("REGISTRY_TEMPLATE_ID").expect("REGISTRY_TEMPLATE_ID not set");
    let uri = Uri {
        value: format!("worker://{template_id}/{}", "registry"),
    };

    stub_registry::Api::new(&uri)
}

// TODO: Actually calculate available balance given orders

const ZERO: u64 = 0u64;
fn available_balance(asset_id: u64) -> u64 {
    with_state(|state| match state.balances.get(&asset_id) {
        Some(balance) => *balance,
        None => ZERO,
    })
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
        let assets = get_registry().get_assets();
        with_state(|state| {
            state
                .balances
                .iter()
                .filter_map(|(asset_id, balance)| {
                    match assets.iter().find(|asset| asset.id == *asset_id) {
                        Some(asset) => Some(AssetBalance {
                            asset: asset.clone(),
                            balance: *balance,
                            available_balance: available_balance(*asset_id),
                        }),
                        None => None,
                    }
                })
                .collect()
        })
    }

    fn place_order(order: Order) -> Result<OrderStatus, Error> {
        todo!()
    }

    fn deposit(asset_id: u64, amount: u64) -> AssetBalance {
        let assets = get_registry().get_assets();
        with_state(|state| {
            let balance = state.balances.entry(asset_id).or_insert_with(|| ZERO);
            *balance += amount;
            match assets.iter().find(|asset| asset.id == asset_id) {
                Some(asset) => AssetBalance {
                    asset: asset.clone(),
                    balance: state.balances.get(&asset_id).unwrap().clone(),
                    available_balance: available_balance(asset_id),
                },
                None => panic!("No asset_id {}", asset_id),
            }
        })
    }

    fn withdraw(asset_id: u64, amount: u64) -> Result<AssetBalance, Error> {
        let assets = get_registry().get_assets();
        let available = available_balance(asset_id);
        if available <= amount {
            Err(InsufficientFunds(available))
        } else {
            with_state(|state| {
                state
                    .balances
                    .entry(asset_id)
                    .and_modify(|amt| *amt -= amount);
                match assets.iter().find(|asset| asset.id == asset_id) {
                    Some(asset) => Ok(AssetBalance {
                        asset: asset.clone(),
                        balance: state.balances.get(&asset_id).unwrap().clone(),
                        available_balance: available_balance(asset_id),
                    }),
                    None => panic!("No asset_id {}", asset_id),
                }
            })
        }
    }
}
