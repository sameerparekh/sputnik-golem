use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::client::WsConnect,
};
use alloy::eips::BlockNumberOrTag;
use alloy::rpc::types::eth::BlockTransactions;
use ethers::prelude::coins_bip39::Mnemonic;
use ethers::prelude::Middleware;
use ethers::signers::coins_bip39::English;
use ethers::utils::hex::encode;
use futures_util::StreamExt;
use hdwallet::ExtendedPrivKey;

#[tokio::main]
pub async fn main() {
    // Create a Seed from the mnemonic and passphrase
    let mnemonic: Mnemonic<English> = Mnemonic::new_from_phrase("weather creek place resemble fitness rebel what artwork devote exclude goat paper").unwrap();
    let seed = mnemonic.to_seed(None).unwrap();

    let priv_key = ExtendedPrivKey::with_seed(&seed).unwrap();
    println!("Private Key: {:?}", encode(priv_key.private_key.as_ref()));
    println!("Chain Code {:?}", encode(priv_key.chain_code));
    let rpc_url = "wss://eth-sepolia.g.alchemy.com/v2/C2AMPkL7J84rizAWoLcCt5rTflAU0tGY";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

    let sub = provider.subscribe_blocks().await.unwrap();

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream();

    // Subscribe to new blocks
    // Process new blocks as they are mined
    loop {
        let block = stream.next().await.unwrap();
        println!("Block: {:?}", block.header.number);
        let by_number = provider.get_block_by_number(BlockNumberOrTag::Number(block.header.number.unwrap()), true).await.unwrap().unwrap();
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


            // Check if the transaction is a contract creation or a normal transaction
            if let Some(to) = tx.to {
                println!("Deposit received: {:?} {:?} {:?}", tx.hash, tx.value.to_base_be(10000000000000000000).collect::<Vec<_>>(), to);
                let amount = tx.value.to_base_be(10000000000000000000).collect::<Vec<_>>();
                if amount.len() > 1 {
                    println!("Deposit too big to fit in u64, ignoring =)")
                } else {
                    // Call the golem monitor
                }
                // Check if the transaction is sent to the target address
                // if to == address_to_monitor {}
            }
        }
    }
}
