use std::cell::RefCell;
use std::collections::HashMap;

use crate::bindings::exports::sputnik::accountant::api::{
    AssetBalance, Error, Guest, Order, OrderStatus,
};
use crate::bindings::exports::sputnik::accountant::api::Error::AlreadyInitialized;
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
        let template_id = std::env::var("PURCHASE_HISTORY_TEMPLATE_ID")
            .expect("PURCHASE_HISTORY_TEMPLATE_ID not set");
        let uri = Uri {
            value: format!("worker://{template_id}/{}", "blah"),
        };

        // Connecting to the remote worker and invoking it
        let registry = stub_registry::Api::new(&uri);
        registry.get_spot_pairs();
        vec![]
    }

    fn place_order(order: Order) -> Result<OrderStatus, Error> {
        todo!()
    }

    fn deposit(asset: u64, amount: u64) -> AssetBalance {
        todo!()
    }

    fn withdraw(asset: u64, amount: u64) -> Result<AssetBalance, Error> {
        todo!()
    }
}
