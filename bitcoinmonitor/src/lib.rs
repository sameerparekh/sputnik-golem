use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;

use bitcoin::bip32::{ChildNumber, Xpub};
use bitcoin::PublicKey;
use bitcoincore_rpc::{bitcoin, RpcApi};
use bitcoincore_rpc::bitcoin::{Address, SignedAmount};
use bitcoincore_rpc::bitcoin::address::{NetworkChecked, NetworkUnchecked};
use bitcoincore_rpc::bitcoin::secp256k1::Secp256k1;
use bitcoincore_rpc::bitcoincore_rpc_json::GetTransactionResultDetailCategory::{Receive, Send};
use bitcoincore_rpc::json::ImportDescriptors;
use secp256k1_sys::types::AlignedType;

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
            let rpc = bitcoincore_rpc::Client::new(
                "http://127.0.0.1:18332",
                bitcoincore_rpc::Auth::UserPass("sameer".to_string(), "test".to_string()),
            )
                .unwrap();

            let transactions = rpc.list_transactions(None, Some(300), None, Some(true)).unwrap();
            transactions.into_iter().for_each(|tx| {
                if let Some(address) = tx.detail.address {
                    if let Some(trader) = state.address_map.get(&address) {
                        if tx.detail.category == Receive {
                            let amount = tx.detail.amount - tx.detail.fee.unwrap_or(SignedAmount::ZERO);
                            // let sats = amount.to_sat();
                            log::info!("Depositing {} to {}", amount, trader);
                            // TODO do the deposit to the accountant
                        }
                    }
                }
            });
        })
    }

    fn new_address_for_trader(trader: u64) -> String {
        with_state(|state| {
            let rpc = bitcoincore_rpc::Client::new(
                "http://127.0.0.1:18332",
                bitcoincore_rpc::Auth::UserPass("sameer".to_string(), "test".to_string()),
            );

            let configuration = state.configuration.clone().expect("Not initialized");

            let secp = bitcoin::key::Secp256k1::new();

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
            let import = ImportDescriptors {
                descriptor: address.to_string(),
                timestamp: Default::default(),
                active: None,
                range: None,
                next_index: None,
                internal: None,
                label: None,
            };
            println!("Got new address: {}", address);
            rpc.unwrap().import_descriptors(import).expect("Address imported to bitcoind");
            address.to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bip0039::{Count, English, Mnemonic};
    use bitcoin::bip32::{DerivationPath, Xpriv, Xpub};
    use bitcoin::secp256k1::Secp256k1;
    use bitcoincore_rpc::bitcoin::secp256k1::ffi::types::AlignedType;

    use crate::bindings::exports::sputnik::bitcoinmonitor::api::Guest;
    use crate::Component;

    #[test]
    fn test_tick() {
        let mnemonic: Mnemonic<English> = Mnemonic::from_str("guitar violin alert void long couple siren need rude oyster kit lizard").unwrap();
        println!("mmnemonic: {}", mnemonic);
        // Gets the phrase
        let _phrase = mnemonic.phrase();
        log::info!("Phrase generated: {}", _phrase);

        let seed = mnemonic.to_seed("blah".to_string());

        let network = bitcoin::Network::Testnet;

        let secp = Secp256k1::new();

        // calculate root key from seed
        let root = Xpriv::new_master(network, &seed).unwrap();
        log::info!("Root key: {}", root);

        // derive child xpub
        let path = DerivationPath::from_str("m/84h/0h/0h").unwrap();
        let child = root.derive_priv(&secp, &path).unwrap();
        log::info!("Child at {}: {}", path, child);

        let xpub = Xpub::from_priv(&secp, &child);
        log::info!("Public key at {}: {}", path, xpub);


        env_logger::init();
        <Component as Guest>::initialize(xpub.to_string(), 1).expect("init success");
        <Component as Guest>::new_address_for_trader(1);
        <Component as Guest>::new_address_for_trader(2);
        <Component as Guest>::tick()
    }
}
