use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::client::WsConnect,
};
use alloy::eips::BlockNumberOrTag;
use alloy::hex::encode;
use alloy::rpc::types::eth::BlockTransactions;
use alloy::signers::wallet::coins_bip39::{English, Mnemonic};
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


                // Check if the transaction is a contract creation or a normal transaction
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
