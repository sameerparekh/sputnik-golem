use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::str::FromStr;

use ethers::prelude::*;
use ethers::prelude::coins_bip39::Mnemonic;
use ethers::utils::hex::hex::decode;
use hdwallet::*;
use secp256k1::{Secp256k1, SecretKey};

use crate::bindings::exports::sputnik::ethereummonitor::api::Guest;

mod bindings;

struct State {
    address_idx: u64,
    address_map: HashMap<String, u64>,
    block_height: Option<u64>,
}

#[derive(Clone)]
struct Configuration {}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        address_idx: 1
        address_map: HashMap::new(),
        block_height: None,
    });
}
fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

struct Component;

impl Guest for Component {
    fn process_deposit(address: String, amount: u64, asset_id: u64, block_height: u64) {
        with_state(|state| {
            if let Some(bh) = state.block_height {
                if bh >= block_height {
                    println!("Already seen block {block_height}");
                }
            } else {
                state.block_height = Some(block_height);
                if let Some(trader) = state.address_map.get(&address.to_lowercase()) {
                    // log::info!("Depositing {} to {}", amount, trader);
                    // TODO do the deposit to the accountant
                }
            }
        });
    }


    fn new_address_for_trader(trader: u64) -> String {
        with_state(|state| {
            let private_key_str = env::var("PRIVATE_KEY").unwrap();
            let chain_code_str = env::var("CHAIN_CODE").unwrap();

            let xpriv = ExtendedPrivKey {
                private_key: SecretKey(<[u8; 32]>::try_from(decode(private_key_str).unwrap()).unwrap()),
                chain_code: decode(chain_code_str).unwrap(),
            };

            let derived_key = xpriv.derive_private_key(state.address_idx).unwrap();
            let address = ethers::utils::secret_key_to_address(derived_key.private_key);
            state.address_idx += 1;
            state.address_map.insert(address, trader);
        })
    }
}

#[cfg(test)]
mod tests {}
