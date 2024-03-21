use std::cell::RefCell;
use std::collections::HashMap;

use crate::bindings::exports::sputnik::accountant::api::{AssetBalance, Error, Guest, Order, OrderStatus};

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

impl Guest for Component {
    fn get_balances() -> Vec<AssetBalance> {
        todo!()
    }

    fn place_order(order: Order) -> Result<OrderStatus, Error> {
        todo!()
    }
}
