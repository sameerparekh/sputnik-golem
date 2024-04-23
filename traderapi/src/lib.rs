use std::cell::RefCell;
use std::env;

use chrono::Utc;
use mockall::automock;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::bindings::exports::sputnik::traderapi::api::Error;
use crate::bindings::exports::sputnik::traderapi::api::Error::InternalError;
use crate::bindings::exports::sputnik::traderapi::api::{
    AssetBalance, Guest, Order, OrderAndStatus,
};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::sputnik::accountant::api::{
    Error as AccountantError, Order as AccountantOrder,
};
use crate::bindings::sputnik::accountant_stub::stub_accountant;
use crate::bindings::sputnik::accountant_stub::stub_accountant::OrderStatus;
use crate::bindings::sputnik::ids_stub::stub_ids;
use crate::bindings::sputnik::matching_engine::api::Order as EngineOrder;
use crate::bindings::sputnik::registry::api::Trader;

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
    fn get_ids(&self) -> stub_ids::Api;

    fn get_accountant(&self, trader_id: u64) -> stub_accountant::Api;

    fn get_balances(&self, trader_id: u64) -> Vec<AssetBalance>;
    fn get_new_id(&self) -> u64;

    fn place_order(
        &self,
        trader_id: u64,
        order: AccountantOrder,
    ) -> Result<OrderStatus, AccountantError>;
    fn get_timestamp(&self) -> u64;

    fn get_orders(&self, trader_id: u64) -> Vec<OrderAndStatus>;
}

pub struct ExternalServiceApiProd;

impl ExternalServiceApi for ExternalServiceApiProd {
    fn get_ids(&self) -> stub_ids::Api {
        let template_id = env::var("IDS_TEMPLATE_ID").expect("IDS_TEMPLATE_ID not set");
        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT NOT SET");
        let uri = Uri {
            value: format!("worker://{template_id}/{environment}"),
        };

        stub_ids::Api::new(&uri)
    }

    fn get_accountant(&self, trader_id: u64) -> stub_accountant::Api {
        let template_id =
            env::var("ACCOUNTANT_TEMPLATE_ID").expect("ACCOUNTANT_TEMPLATE_ID not set");
        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT NOT SET");
        let uri = Uri {
            value: format!("worker://{template_id}/{environment}-{trader_id}"),
        };

        stub_accountant::Api::new(&uri)
    }

    fn get_balances(&self, trader_id: u64) -> Vec<AssetBalance> {
        self.get_accountant(trader_id).get_balances()
    }

    fn get_new_id(&self) -> u64 {
        self.get_ids().get_new_id()
    }

    fn place_order(
        &self,
        trader_id: u64,
        order: AccountantOrder,
    ) -> Result<OrderStatus, AccountantError> {
        self.get_accountant(trader_id).place_order(order)
    }

    fn get_timestamp(&self) -> u64 {
        Utc::now().timestamp() as u64
    }

    fn get_orders(&self, trader_id: u64) -> Vec<OrderAndStatus> {
        self.get_accountant(trader_id).get_orders()
    }
}

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}
impl Guest for Component {
    fn get_balances(trader: u64) -> Vec<AssetBalance> {
        with_state(|state| state.external_service_api.get_balances(trader))
    }

    fn get_orders(trader: u64) -> Vec<OrderAndStatus> {
        with_state(|state| state.external_service_api.get_orders(trader))
    }

    fn place_order(trader: u64, order: Order) -> Result<u64, Error> {
        with_state(|state| {
            let order_id = state.external_service_api.get_new_id();
            let timestamp = state.external_service_api.get_timestamp();

            match state.external_service_api.place_order(
                trader,
                AccountantOrder {
                    id: order_id,
                    spot_pair: order.spot_pair,
                    timestamp,
                    side: order.side,
                    price: order.price,
                    size: order.size,
                },
            ) {
                Err(err) => Err(InternalError(err)),
                Ok(OrderStatus { id, .. }) => Ok(id),
            }
        })
    }
}

#[cfg(test)]
mod tests {}
