use std::cell::RefCell;

use crate::bindings::exports::sputnik::admin_api::api::Guest;

mod bindings;

struct Component;

struct State {
    last_id: u64,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State { last_id: 0u64 });
}

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}
impl Guest for Component {
    fn create_asset() -> u64 {
        todo!()
    }

    fn create_spot_pair() -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
