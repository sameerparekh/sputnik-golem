use std::cell::RefCell;
use std::collections::HashMap;

use crate::bindings::{Asset, Guest};
use crate::bindings::{Error, SpotPair};

mod bindings;

struct Component;

struct State {
    assets: HashMap<u64, Asset>,
    spot_pairs: HashMap<u64, SpotPair>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        assets: HashMap::new(),
        spot_pairs: HashMap::new(),
    });
}

impl Guest for Component {
    fn get_assets() -> Vec<Asset> {
        todo!()
    }

    fn get_spot_pairs() -> Vec<SpotPair> {
        todo!()
    }

    fn add_asset(asset: Asset) -> Result<Asset, Error> {
        todo!()
    }

    fn add_spot_pair(pair: SpotPair) -> Result<SpotPair, Error> {
        todo!()
    }
}
