use std::cell::RefCell;
use std::collections::HashMap;

use crate::bindings::exports::sputnik::registry::api::{
    Asset, Error, Guest, HydratedSpotPair, SpotPair, Trader,
};

mod bindings;

struct Component;

struct State {
    assets: HashMap<u64, Asset>,
    spot_pairs: HashMap<u64, SpotPair>,
    traders: HashMap<u64, Trader>,
}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.decimals == other.decimals
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        assets: HashMap::new(),
        spot_pairs: HashMap::new(),
        traders: HashMap::new(),
    });
}

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

impl SpotPair {
    fn hydrate(&self, state: &State) -> Result<HydratedSpotPair, Error> {
        let numerator = state.assets.get(&self.numerator_id);
        let denominator = state.assets.get(&self.denominator_id);
        match (numerator, denominator) {
            (None, _) => Err(Error::NoSuchAsset(self.numerator_id)),
            (_, None) => Err(Error::NoSuchAsset(self.denominator_id)),
            (Some(num), Some(denom)) => Ok(HydratedSpotPair {
                id: self.id,
                name: self.name.clone(),
                numerator: num.clone(),
                denominator: denom.clone(),
            }),
        }
    }
}

impl Guest for Component {
    fn get_assets() -> Vec<Asset> {
        with_state(|state| state.assets.values().cloned().collect())
    }

    fn get_spot_pairs() -> Vec<HydratedSpotPair> {
        with_state(|state| {
            let spot_pair_values: Vec<SpotPair> = state.spot_pairs.values().cloned().collect();
            spot_pair_values
                .iter()
                .flat_map(move |pair| match pair.hydrate(state) {
                    Ok(hydrated_pair) => Some(hydrated_pair.clone()),
                    Err(_) => None,
                })
                .collect()
        })
    }

    fn get_traders() -> Vec<Trader> {
        todo!()
    }

    fn add_asset(asset: Asset) -> Result<Asset, Error> {
        with_state(|state| {
            if state.assets.contains_key(&asset.id) {
                Err(Error::DuplicateId(asset.id))
            } else {
                state.assets.insert(asset.id, asset.clone());
                Ok(asset)
            }
        })
    }

    fn add_spot_pair(pair: SpotPair) -> Result<HydratedSpotPair, Error> {
        let result = with_state(|state| {
            if state.spot_pairs.contains_key(&pair.id) {
                Err(Error::DuplicateId(pair.id))
            } else {
                let hydrated_pair = pair.hydrate(state);
                if hydrated_pair.is_ok() {
                    state.spot_pairs.insert(pair.id, pair.clone());
                }
                hydrated_pair
            }
        });
        // create_matching_engine(pair.id.clone());
        result
    }

    fn add_trader(trader: Trader) -> Result<Trader, Error> {
        with_state(|state| {
            if state.traders.contains_key(&trader.id) {
                Err(Error::DuplicateId(trader.id))
            } else {
                state.traders.insert(trader.id, trader.clone());
                Ok(trader)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use assert_unordered::assert_eq_unordered;

    use crate::bindings::exports::sputnik::registry::api::{
        Asset, Guest, HydratedSpotPair, SpotPair,
    };
    use crate::Component;

    fn populate() {
        <Component as Guest>::add_asset(Asset {
            id: 0,
            name: "BTC".to_string(),
            decimals: 8,
        })
        .expect("add asset returns ok");

        <Component as Guest>::add_asset(Asset {
            id: 1,
            name: "USD".to_string(),
            decimals: 4,
        })
        .expect("add asset returns ok");

        <Component as Guest>::add_spot_pair(SpotPair {
            id: 2,
            name: "BTCUSD".to_string(),
            numerator_id: 0,
            denominator_id: 1,
        })
        .expect("add spotpair returns ok");
    }

    impl PartialEq for SpotPair {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
                && self.name == other.name
                && self.denominator_id == other.denominator_id
                && self.numerator_id == other.numerator_id
        }
    }

    impl PartialEq for HydratedSpotPair {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
                && self.name == other.name
                && self.denominator == other.denominator
                && self.numerator == other.numerator
        }
    }

    #[test]
    fn get_assets_returns_assets() {
        populate();
        let assets = <Component as Guest>::get_assets();
        assert_eq_unordered!(
            &assets,
            &vec![
                Asset {
                    id: 0,
                    name: "BTC".to_string(),
                    decimals: 8
                },
                Asset {
                    id: 1,
                    name: "USD".to_string(),
                    decimals: 4
                }
            ]
        );
    }

    #[test]
    fn get_spot_pair_returns_spot_pair() {
        populate();
        let spot_pairs = <Component as Guest>::get_spot_pairs();
        assert_eq!(
            spot_pairs,
            vec![HydratedSpotPair {
                id: 2,
                name: "BTCUSD".to_string(),
                numerator: Asset {
                    id: 0,
                    name: "BTC".to_string(),
                    decimals: 8,
                },
                denominator: Asset {
                    id: 1,
                    name: "USD".to_string(),
                    decimals: 4,
                }
            }]
        );
    }
}
