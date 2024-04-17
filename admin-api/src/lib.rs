use std::cell::RefCell;

use mockall::automock;
// use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::bindings::exports::sputnik::admin_api::api::Guest;
use crate::bindings::sputnik::registry::api::{Asset, HydratedSpotPair, SpotPair};
use crate::bindings::sputnik::registry_stub::stub_registry;

mod bindings;

struct Component;

struct State {
    last_id: u64,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State { last_id: 0u64 });
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
// fn create_matching_engine(spot_pair_id: u64) {
//     let client = Client::new();
//     let template_id = env::var("MATCHING_ENGINE_TEMPLATE_ID").unwrap();
//     let url = format!(
//         "https://release.api.golem.cloud/v1/templates/{}/workers",
//         template_id
//     );
//     let body = CreateWorkerBody::new(format!("{}", spot_pair_id));
//     let token = env::var("GOLEM_TOKEN_SECRET").unwrap();
//     let response = client
//         .post(url)
//         .json(&body)
//         .header("Authorization", format!("Bearer {}", token))
//         .send()
//         .unwrap();
//     assert!(response.status().is_success());
// }

#[automock]
trait ExternalServiceApi {
    // fn get_registry(&self) -> stub_registry::Api;
    // fn get_ids(&self) -> stub_ids::Api;
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
