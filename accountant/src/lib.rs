use std::cell::RefCell;
use std::collections::HashMap;

use crate::bindings::exports::sputnik::accountant::api::{
    AssetBalance, Error, Guest, Order, OrderStatus,
};

mod bindings;

struct Component;

struct State {
    balances: HashMap<u64, u64>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        balances: HashMap::new(),
    });
}
fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

impl Guest for Component {
    fn get_balances() -> Vec<AssetBalance> {
        todo!()
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
