use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::str::FromStr;

use bip32::{ChildNumber, PublicKey, XPrv};
use ethers_core::k256::elliptic_curve::weierstrass::add;
use ethers_core::utils::hex::decode;
use ethers_core::utils::hex::hex::encode;
use secp256k1::{Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

use crate::bindings::exports::sputnik::ethereummonitor::api::Guest;

mod bindings;

struct State {
    address_idx: u32,
    address_map: HashMap<String, u64>,
    block_height: Option<u64>,
}

#[derive(Clone)]
struct Configuration {}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        address_idx: 1,
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
            // let chain_code_str = env::var("CHAIN_CODE").unwrap();

            let xpriv: XPrv = private_key_str.parse().unwrap();

            let derived_key = xpriv.derive_child(ChildNumber(state.address_idx)).unwrap();
            let pubkey = derived_key.public_key();

            let mut hasher = Keccak256::new();
            hasher.update(&pubkey.public_key().to_bytes());
            let result = hasher.finalize();

            // Take the last 20 bytes as the Ethereum address
            let mut address = [0u8; 20];
            address.copy_from_slice(&result[12..32]);
            state.address_idx += 1;
            state.address_map.insert(encode(address), trader);
            encode(address)
        })
    }
}

#[cfg(test)]
mod tests {}
