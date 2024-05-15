use std::cell::RefCell;
use std::env;

use mockall::automock;

use crate::bindings::exports::sputnik::adminapi::api::{Error, Guest, Trader};
use crate::bindings::exports::sputnik::adminapi::api::Error::{
    Internal, UnableToMakeAccountant, UnableToMakeEngine,
};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::sputnik::accountant_stub::stub_accountant;
use crate::bindings::sputnik::ids_stub::stub_ids;
use crate::bindings::sputnik::matching_engine_stub::stub_matching_engine;
use crate::bindings::sputnik::registry::api::{
    Asset, Error as RegistryError, HydratedSpotPair, SpotPair,
};
use crate::bindings::sputnik::registry_stub::stub_registry;

mod bindings;

struct Component;

struct State {
    external_service_api: Box<dyn ExternalServiceApi>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State { external_service_api: Box::new(ExternalServiceApiProd) });
}

#[automock]
trait ExternalServiceApi {
    fn get_registry(&self) -> stub_registry::Api;
    fn get_ids(&self) -> stub_ids::Api;

    fn get_new_id(&self) -> u64;

    fn create_asset(&self, asset: &Asset) -> Result<Asset, RegistryError>;

    fn create_spot_pair(&self, spot_pair: &SpotPair) -> Result<HydratedSpotPair, RegistryError>;

    fn create_trader(&self, trader: &Trader) -> Result<Trader, RegistryError>;
    fn create_matching_engine(&self, spot_pair_id: u64) -> Result<(), Error>;
    fn create_accountant(&self, trader_id: u64) -> Result<(), Error>;
}

pub struct ExternalServiceApiProd;

impl ExternalServiceApi for ExternalServiceApiProd {
    fn get_registry(&self) -> stub_registry::Api {
        let component_id =
            env::var("REGISTRY_COMPONENT_ID").expect("REGISTRY_COMPONENT_ID not set");
        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT NOT SET");
        let uri = Uri {
            value: format!("worker://{component_id}/{environment}"),
        };

        stub_registry::Api::new(&uri)
    }

    fn get_ids(&self) -> stub_ids::Api {
        let component_id = env::var("IDS_COMPONENT_ID").expect("IDS_COMPONENT_ID not set");
        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT NOT SET");
        let uri = Uri {
            value: format!("worker://{component_id}/{environment}"),
        };

        stub_ids::Api::new(&uri)
    }

    fn get_new_id(&self) -> u64 {
        self.get_ids().get_new_id()
    }

    fn create_asset(&self, asset: &Asset) -> Result<Asset, RegistryError> {
        self.get_registry().add_asset(asset)
    }

    fn create_spot_pair(&self, spot_pair: &SpotPair) -> Result<HydratedSpotPair, RegistryError> {
        self.get_registry().add_spot_pair(spot_pair)
    }

    fn create_trader(&self, trader: &Trader) -> Result<Trader, RegistryError> {
        self.get_registry().add_trader(trader)
    }

    fn create_matching_engine(&self, spot_pair_id: u64) -> Result<(), Error> {
        let component_id =
            env::var("MATCHING_ENGINE_COMPONENT_ID").expect("MATCHING_ENGINE_COMPONENT_ID not set");
        let accountant_component_id =
            env::var("ACCOUNTANT_COMPONENT_ID").expect("ACCOUNTANT_COMPONENT_ID not set");

        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT NOT SET");
        let uri = Uri {
            value: format!("worker://{component_id}/{environment}-{spot_pair_id}"),
        };

        let engine = stub_matching_engine::Api::new(&uri);
        match engine.init(accountant_component_id.as_str(), environment.as_str()) {
            Ok(_) => Ok(()),
            Err(err) => Err(UnableToMakeEngine(format!("{}", err))),
        }
    }

    fn create_accountant(&self, trader_id: u64) -> Result<(), Error> {
        let matching_engine_component_id =
            env::var("MATCHING_ENGINE_COMPONENT_ID").expect("MATCHING_ENGINE_COMPONENT_ID not set");
        let registry_component_id =
            env::var("REGISTRY_COMPONENT_ID").expect("REGISTRY_COMPONENT_ID not set");

        let component_id =
            env::var("ACCOUNTANT_COMPONENT_ID").expect("ACCOUNTANT_COMPONENT_ID not set");
        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT not set");

        let uri = Uri {
            value: format!("worker://{component_id}/{environment}-{trader_id}"),
        };
        let accountant = stub_accountant::Api::new(&uri);
        match accountant.initialize(
            trader_id,
            matching_engine_component_id.as_str(),
            registry_component_id.as_str(),
            environment.as_str(),
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(UnableToMakeAccountant(format!("{}", err))),
        }
    }
}

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

impl Guest for Component {
    fn create_asset(name: String, decimals: u8, token_address: String) -> Result<Asset, Error> {
        with_state(|state| {
            let asset_id = state.external_service_api.get_new_id();
            match state.external_service_api.create_asset(&Asset {
                id: asset_id,
                name,
                decimals,
            }) {
                Ok(result) => Ok(result),
                Err(err) => Err(Internal(format!("{}", err))),
            }
        })
    }

    fn create_spot_pair(
        name: String,
        numerator: u64,
        denominator: u64,
    ) -> Result<HydratedSpotPair, Error> {
        with_state(|state| {
            let pair_id = state.external_service_api.get_new_id();
            state.external_service_api.create_matching_engine(pair_id)?;
            match state.external_service_api.create_spot_pair(&SpotPair {
                id: pair_id,
                name,
                numerator_id: numerator,
                denominator_id: denominator,
            }) {
                Ok(result) => Ok(result),
                Err(err) => Err(Internal(format!("{}", err))),
            }
        })
    }

    fn create_trader(name: String) -> Result<Trader, Error> {
        with_state(|state| {
            let trader_id = state.external_service_api.get_new_id();
            state.external_service_api.create_accountant(trader_id)?;
            match state.external_service_api.create_trader(&Trader {
                id: trader_id,
                name,
            }) {
                Ok(result) => Ok(result),
                Err(err) => Err(Internal(format!("{}", err))),
            }
        })
    }
}

#[cfg(test)]
mod tests {}
