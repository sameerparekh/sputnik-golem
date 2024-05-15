use std::str::FromStr;

use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::client::WsConnect,
};
use alloy::eips::BlockNumberOrTag;
use alloy::rpc::types::eth::BlockTransactions;
use alloy::signers::wallet::coins_bip39::{English, Mnemonic};
use bip32::{ChildNumber, Prefix, PublicKey, XPrv};
use futures_util::StreamExt;
use sha3::{Digest, Keccak256};

#[tokio::main]
pub async fn main() {
    // Create a Seed from the mnemonic and passphrase
    let mnemonic: Mnemonic<English> = Mnemonic::new_from_phrase("weather creek place resemble fitness rebel what artwork devote exclude goat paper").unwrap();
    let seed = mnemonic.to_seed(None).unwrap();

    let priv_key = XPrv::new(&seed).unwrap();
    let encoded_key = priv_key.to_extended_key(Prefix::XPRV).to_string();
    println!("Private Key: {:?}", encoded_key);
    // println!("Chain Code {:?}", encode(priv_key.chain_code));

    // let chain_code_str = env::var("CHAIN_CODE").unwrap();

    let xpriv: XPrv = XPrv::from_str(&encoded_key).unwrap();

    let derived_key = xpriv.derive_child(ChildNumber(1)).unwrap();
    let pubkey = derived_key.public_key();

    let mut hasher = Keccak256::new();
    hasher.update(pubkey.public_key().to_bytes());
    let result = hasher.finalize();

    // Take the last 20 bytes as the Ethereum address
    let mut address = [0u8; 20];
    address.copy_from_slice(&result[12..32]);
    // state.address_idx += 1;
    // state.address_map.insert(alloy::hex::hex::encode(address), trader);
    println!("Address: {}", alloy::hex::hex::encode(address));

    let rpc_url = "wss://eth-sepolia.g.alchemy.com/v2/C2AMPkL7J84rizAWoLcCt5rTflAU0tGY";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

    let sub = provider.subscribe_blocks().await.unwrap();

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream();

    // Subscribe to new blocks
    // Process new blocks as they are mined
    let mut last_block = 5897186; // get this from golem worker

    loop {
        let block = stream.next().await.unwrap();
        println!("Block: {:?}", block.header.number);
        let number = block.header.number.unwrap();
        let blocks = (last_block + 1)..=number;
        for num in blocks {
            println!("Getting block {num}");
            let by_number = provider.get_block_by_number(BlockNumberOrTag::Number(num), true).await.unwrap().unwrap();
            let txes = match by_number.transactions {
                BlockTransactions::Full(txes) => txes,
                BlockTransactions::Hashes(hashes) => futures::future::join_all(hashes.iter().map(|hash| async {
                    provider.get_transaction_by_hash(*hash).await.unwrap()
                }).collect::<Vec<_>>()).await,
                BlockTransactions::Uncle => {
                    println!("Uncle");
                    vec![]
                }
            };


            for tx in txes {
                // println!("tx: {:?}", tx);
                // let receipt = provider.get_transaction_receipt(tx.hash).await.unwrap().unwrap();


                if let Some(to) = tx.to {
                    println!("Deposit received: {:?} {:?} {:?}", tx.hash, tx.value.to_base_be(10000000000000000000).collect::<Vec<_>>(), to);
                    let amount = tx.value.to_base_be(10000000000000000000).collect::<Vec<_>>();
                    if amount.len() > 1 {
                        println!("Deposit too big to fit in u64, ignoring =)")
                    } else {
                        // Call the golem monitor with the deposit
                    }
                    // Check if the transaction is sent to the target address
                    // if to == address_to_monitor {}
                }
            }
            // call the golem monitor with the end block call
        }
        last_block = number;
    }
}
