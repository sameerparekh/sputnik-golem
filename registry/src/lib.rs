use std::cell::RefCell;
use std::collections::HashMap;

use crate::bindings::exports::sputnik::registry::api::{
    Asset, AssetMismatchDetails, Error, Guest, SpotPair,
};

mod bindings;

struct Component;

struct State {
    assets: HashMap<u64, Asset>,
    spot_pairs: HashMap<u64, SpotPair>,
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
    });
}

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

impl Guest for Component {
    fn get_assets() -> Vec<Asset> {
        with_state(|state| state.assets.values().cloned().collect())
    }

    fn get_spot_pairs() -> Vec<SpotPair> {
        with_state(|state| state.spot_pairs.values().cloned().collect())
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

    fn add_spot_pair(pair: SpotPair) -> Result<SpotPair, Error> {
        // TODO: Create a matching engine for this pair
        with_state(|state| {
            if state.spot_pairs.contains_key(&pair.id) {
                Err(Error::DuplicateId(pair.id))
            } else {
                let numerator = state.assets.get(&pair.numerator.id);
                let denominator = state.assets.get(&pair.denominator.id);
                match (numerator, denominator) {
                    (None, _) => Err(Error::NoSuchAsset(pair.numerator.id)),
                    (_, None) => Err(Error::NoSuchAsset(pair.denominator.id)),
                    (Some(num), Some(denom)) => {
                        if *num != pair.numerator {
                            Err(Error::AssetMismatch(AssetMismatchDetails {
                                incoming_asset: pair.numerator,
                                expected_asset: num.clone(),
                            }))
                        } else if *denom != pair.denominator {
                            Err(Error::AssetMismatch(AssetMismatchDetails {
                                incoming_asset: pair.denominator,
                                expected_asset: denom.clone(),
                            }))
                        } else {
                            state.spot_pairs.insert(pair.id, pair.clone());
                            Ok(pair)
                        }
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use assert_unordered::assert_eq_unordered;

    use crate::bindings::exports::sputnik::registry::api::{Asset, Guest, SpotPair};
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
            numerator: Asset {
                id: 1,
                name: "USD".to_string(),
                decimals: 4,
            },
            denominator: Asset {
                id: 1,
                name: "USD".to_string(),
                decimals: 4,
            },
        })
        .expect("add spotpair returns ok");
    }

    impl PartialEq for SpotPair {
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
            vec![SpotPair {
                id: 2,
                name: "BTCUSD".to_string(),
                numerator: Asset {
                    id: 1,
                    name: "USD".to_string(),
                    decimals: 4,
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
