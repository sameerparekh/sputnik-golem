use std::cell::RefCell;

use crate::bindings::exports::sputnik::ids::api::Guest;

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
    fn get_new_id() -> u64 {
        with_state(|state| {
            state.last_id += 1;
            state.last_id
        })
    }
    fn get_new_ids(qty: u8) -> Vec<u64> {
        with_state(|state| {
            let start = state.last_id + 1;
            let end = start + qty as u64;
            state.last_id = end - 1;
            (start..end).collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::bindings::exports::sputnik::ids::api::Guest;
    use crate::Component;

    #[test]
    fn test_get_id() {
        let first_id = <Component as Guest>::get_new_id();
        let second_id = <Component as Guest>::get_new_id();
        assert_eq!((first_id, second_id), (1, 2));
    }

    #[test]
    fn test_get_ids() {
        let first_id = <Component as Guest>::get_new_id();
        let second_ids = <Component as Guest>::get_new_ids(20);
        let third_id = <Component as Guest>::get_new_id();
        assert_eq!(
            (first_id, second_ids, third_id),
            (
                1,
                vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21],
                22
            )
        );
    }
}
