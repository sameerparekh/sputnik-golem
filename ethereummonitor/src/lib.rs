use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::env;

use alloy_primitives::hex::FromHexError;
use alloy_primitives::Address;
use bip32::{ChildNumber, PublicKey, XPrv};
use mockall::automock;
use sha3::{Digest, Keccak256};

use crate::bindings::exports::sputnik::ethereummonitor::api::Error::{
    InvalidAddress, TokenExists, TxSeen, UnknownAddress, WrongBlock,
};
use crate::bindings::exports::sputnik::ethereummonitor::api::{BlockHeightResponse, Error, Guest};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::sputnik::accountant::api::Error as AccountantError;
use crate::bindings::sputnik::accountant_stub::stub_accountant;
use crate::bindings::sputnik::accountant_stub::stub_accountant::AssetBalance;

mod bindings;

struct State {
    address_idx: u32,
    address_map: HashMap<Address, u64>,
    token_asset_map: HashMap<Address, u64>,
    block_height: u64,
    external_service_api: Box<dyn ExternalServiceApi>,
    txes: HashSet<String>,
}

#[derive(Clone)]
struct Configuration {}

#[automock]
trait ExternalServiceApi {
    fn get_accountant(&self, trader_id: u64) -> stub_accountant::Api;

    fn deposit(
        &self,
        trader_id: u64,
        asset_id: u64,
        amount: u64,
    ) -> Result<AssetBalance, AccountantError>;
}

struct ExternalServiceApiProd;

impl From<AccountantError> for Error {
    fn from(value: AccountantError) -> Self {
        Error::AccountantError(value)
    }
}

impl From<FromHexError> for Error {
    fn from(value: FromHexError) -> Self {
        InvalidAddress(value.to_string())
    }
}

impl ExternalServiceApi for ExternalServiceApiProd {
    fn get_accountant(&self, trader_id: u64) -> stub_accountant::Api {
        let component_id =
            env::var("ACCOUNTANT_COMPONENT_ID").expect("ACCOUNTANT_COMPONENT_ID not set");
        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT NOT SET");
        let uri = Uri {
            value: format!("worker://{component_id}/{environment}-{trader_id}"),
        };

        stub_accountant::Api::new(&uri)
    }

    fn deposit(
        &self,
        trader_id: u64,
        asset_id: u64,
        amount: u64,
    ) -> Result<AssetBalance, AccountantError> {
        self.get_accountant(trader_id).deposit(asset_id, amount)
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        address_idx: 1,
        address_map: HashMap::new(),
        token_asset_map: HashMap::new(),
        block_height: env::var("BLOCK_HEIGHT").expect("BLOCK_HEIGHT not set").parse().unwrap(),
        external_service_api: Box::new(ExternalServiceApiProd),
        txes: HashSet::new(),
    });
}
fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

struct Component;

impl Guest for Component {
    fn process_deposit(
        address: String,
        tx: String,
        amount: u64,
        token_address: String,
        block_height: u64,
    ) -> Result<AssetBalance, Error> {
        with_state(|state| {
            let token_address_parsed: Address = token_address.parse()?;
            if let Some(asset_id) = state.token_asset_map.get(&token_address_parsed) {
                if block_height != state.block_height {
                    Err(WrongBlock(state.block_height))
                } else if state.txes.contains(&tx) {
                    Err(TxSeen(tx))
                } else {
                    state.txes.insert(tx);
                    let parsed_address: Address = address.parse()?;
                    if let Some(trader) = state.address_map.get(&parsed_address) {
                        Ok(state
                            .external_service_api
                            .deposit(*trader, *asset_id, amount)?)
                    } else {
                        Err(UnknownAddress(address))
                    }
                }
            } else {
                Err(UnknownAddress(token_address))
            }
        })
    }

    fn complete_block(block: u64) -> Result<(), Error> {
        with_state(|state| {
            if block != state.block_height {
                Err(WrongBlock(state.block_height))
            } else {
                state.block_height = block + 1;
                state.txes.clear();
                Ok(())
            }
        })
    }

    fn block_height() -> BlockHeightResponse {
        with_state(|state| BlockHeightResponse {
            height: state.block_height,
        })
    }

    fn new_address_for_trader(trader: u64) -> String {
        with_state(|state| {
            let private_key_str = env::var("PRIVATE_KEY").expect("PRIVATE_KEY is not set");

            let xpriv: XPrv = private_key_str
                .parse()
                .expect("PRIVATE_KEY parses correctly");

            let derived_key = xpriv
                .derive_child(ChildNumber(state.address_idx))
                .unwrap_or_else(|_| panic!("derive_child works for index {}", state.address_idx));
            let pubkey = derived_key.public_key();

            let mut hasher = Keccak256::new();
            hasher.update(pubkey.public_key().to_bytes());
            let result = hasher.finalize();

            // Take the last 20 bytes as the Ethereum address
            let mut address = [0u8; 20];
            address.copy_from_slice(&result[12..32]);

            state.address_idx += 1;
            state.address_map.insert(Address::from(address), trader);
            Address::from(address).to_string()
        })
    }

    fn add_token(address: String, asset_id: u64) -> Result<(), Error> {
        with_state(|state| {
            let parsed_address: Address = address.parse()?;
            if let Entry::Vacant(e) = state.token_asset_map.entry(parsed_address) {
                e.insert(asset_id);
                Ok(())
            } else {
                Err(TokenExists(address))
            }
        })
    }
}

#[cfg(test)]
mod tests {}
