use std::cell::RefCell;
use std::env;

use mockall::automock;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::bindings::exports::sputnik::adminapi::api::{Error, Guest};
use crate::bindings::exports::sputnik::adminapi::api::Error::Internal;
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::sputnik::ids_stub::stub_ids;
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

#[derive(Serialize, Deserialize, Debug)]
struct CreateWorkerBody {
    name: String,
    args: Vec<String>,
    env: Vec<Vec<String>>,
}

impl CreateWorkerBody {
    fn new(name: String) -> CreateWorkerBody {
        CreateWorkerBody {
            name,
            args: Vec::new(),
            env: Vec::new(),
        }
    }
}

#[automock]
trait ExternalServiceApi {
    fn get_registry(&self) -> stub_registry::Api;
    fn get_ids(&self) -> stub_ids::Api;

    fn get_new_id(&self) -> u64;

    fn create_asset(&self, asset: &Asset) -> Result<Asset, RegistryError>;

    fn create_spot_pair(&self, spot_pair: &SpotPair) -> Result<HydratedSpotPair, RegistryError>;

    fn create_matching_engine(&self, spot_pair_id: u64);
}

pub struct ExternalServiceApiProd;

impl ExternalServiceApi for ExternalServiceApiProd {
    fn get_registry(&self) -> stub_registry::Api {
        let template_id = env::var("REGISTRY_TEMPLATE_ID").expect("REGISTRY_TEMPLATE_ID not set");
        let uri = Uri {
            value: format!("worker://{template_id}/{}", "registry"),
        };

        stub_registry::Api::new(&uri)
    }

    fn get_ids(&self) -> stub_ids::Api {
        let template_id = env::var("IDS_TEMPLATE_ID").expect("IDS_TEMPLATE_ID not set");
        let uri = Uri {
            value: format!("worker://{template_id}/{}", "ids"),
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
    fn create_matching_engine(&self, spot_pair_id: u64) {
        let client = Client::new();
        let template_id =
            env::var("MATCHING_ENGINE_TEMPLATE_ID").expect("MATCHING_ENGINE_TEMPLATE_ID not set");
        let url = format!(
            "https://release.api.golem.cloud/v1/templates/{}/workers",
            template_id
        );
        let body = CreateWorkerBody::new(format!("{}", spot_pair_id));
        let token = env::var("GOLEM_TOKEN_SECRET").expect("GOLEM_TOKEN_SECRET not set");
        let _ = client
            .post(url)
            .json(&body)
            .header("Authorization", format!("Bearer {}", token))
            .send();
    }
}

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}
impl Guest for Component {
    fn create_asset(name: String, decimals: u8) -> Result<Asset, Error> {
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
            state.external_service_api.create_matching_engine(pair_id);
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
}

#[cfg(test)]
mod tests {}
