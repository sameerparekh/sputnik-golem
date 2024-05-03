use std::cell::RefCell;
use std::collections::HashMap;

use std::str::FromStr;



use bitcoin::bip32::{ChildNumber, Xpub};
use bitcoin::PublicKey;
use bitcoincore_rpc::{bitcoin, RpcApi};
use bitcoincore_rpc::bitcoin::{Address, SignedAmount};
use bitcoincore_rpc::bitcoin::address::{NetworkChecked, NetworkUnchecked};

use bitcoincore_rpc::bitcoin::secp256k1::ffi::types::AlignedType;
use bitcoincore_rpc::bitcoin::secp256k1::Secp256k1;
use bitcoincore_rpc::bitcoincore_rpc_json::GetTransactionResultDetailCategory::{Receive, Send};

use crate::bindings::exports::sputnik::bitcoinmonitor::api::{Error, Guest};
use crate::bindings::exports::sputnik::bitcoinmonitor::api::Error::{
    AlreadyInitialized, BitcoinError,
};

mod bindings;
struct State {
    address_map: HashMap<Address<NetworkUnchecked>, u64>,
    address_idx: ChildNumber,
    configuration: Option<Configuration>,
    block_height: u64,
}

#[derive(Clone)]
struct Configuration {
    xpub: Xpub,
    btc_asset_id: u64,
    network: bitcoin::Network,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        address_map: HashMap::new(),
        address_idx: ChildNumber::from_normal_idx(0).unwrap(),
        configuration: None,
        block_height: 0,
    });
}
fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(f)
}

struct Component;

impl From<bitcoin::bip32::Error> for Error {
    fn from(value: bitcoin::bip32::Error) -> Self {
        BitcoinError(format!("{}", value))
    }
}
impl Guest for Component {
    fn initialize(xpub: String, btc_asset_id: u64) -> Result<(), Error> {
        with_state(|state| match state.configuration {
            Some(_) => Err(AlreadyInitialized),
            None => {
                state.configuration = Some(Configuration {
                    xpub: Xpub::from_str(xpub.as_str())?,
                    btc_asset_id,
                    network: bitcoin::Network::Testnet,
                });
                Ok(())
            }
        })
    }

    fn tick() {
        with_state(|state| {
            let rpc =
                bitcoincore_rpc::Client::new("http://localhost:8332", bitcoincore_rpc::Auth::None)
                    .unwrap();

            let mut next_block = rpc.get_block_hash(state.block_height).unwrap();
            let mut latest_scanned_block: String = "".to_string();

            let block = rpc.get_block_info(&next_block).unwrap();
            let mut deposits: HashMap<u64, SignedAmount> = HashMap::new();

            if latest_scanned_block != block.hash.to_string() {
                for tx in &block.tx {
                    let transaction = rpc.get_transaction(tx, None).unwrap();
                    transaction
                        .details
                        .iter()
                        .cloned()
                        .for_each(|detail| match detail.address {
                            Some(address) => {
                                if let Some(trader_id) = state.address_map.get(&address) {
                                    match detail.category {
                                        Send => {
                                            let deposit = deposits
                                                .entry(*trader_id)
                                                .or_insert(SignedAmount::ZERO);
                                            *deposit -= detail.amount
                                        }
                                        Receive => {
                                            let deposit = deposits
                                                .entry(*trader_id)
                                                .or_insert(SignedAmount::ZERO);
                                            *deposit += detail.amount
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            None => (),
                        });
                    latest_scanned_block = block.hash.to_string();
                }
            }
            if block.nextblockhash.is_some() && block.confirmations >= 8 {
                log::info!("{}", serde_json::to_string_pretty(&block).unwrap());
                next_block = block.nextblockhash.unwrap();
            } else {
                // Make deposits
                state.block_height = block.height as u64 + 1; // ??
                deposits.iter().for_each(|(_trader, deposit)| {
                    let _sats = deposit.to_sat();
                });
                log::info!("No more blocks");
            }
        })
    }

    fn new_address_for_trader(trader: u64) -> String {
        with_state(|state| {
            let configuration = state.configuration.clone().expect("Not initialized");

            let mut buf: Vec<AlignedType> = Vec::new();
            buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
            let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

            let public_key = configuration
                .xpub
                .derive_pub(&secp, &vec![state.address_idx, state.address_idx])
                .unwrap()
                .public_key;
            let address: Address<NetworkChecked> =
                Address::p2wpkh(&PublicKey::new(public_key), configuration.network).unwrap();
            state
                .address_map
                .insert(address.as_unchecked().clone(), trader);
            state.address_idx = state.address_idx.increment().unwrap();
            address.to_string()
        })
    }
}
