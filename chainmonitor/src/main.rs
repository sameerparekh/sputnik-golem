use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::string::ToString;

use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::client::WsConnect,
};
use alloy::eips::BlockNumberOrTag;
use alloy::primitives::{Address, B256, U256};
use alloy::rpc::types::eth::{BlockTransactions, Transaction};
use alloy::signers::wallet::coins_bip39::{English, Mnemonic};
use bip32::{ChildNumber, Prefix, PublicKey, XPrv};
use clap::{Parser, Subcommand};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Deserialize)]
struct BlockHeight {
    height: u64,
}

#[derive(Serialize)]
struct Deposit {
    address: String,
    tx: String,
    amount: u64,
    token_address: String,
    block_height: u64,
}

impl Display for Deposit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, {}, {}", self.address, self.tx, self.amount, self.token_address, self.block_height)
    }
}

async fn get_last_block(ethereum_monitor_api: &String) -> Result<u64, reqwest::Error> {
    let response = reqwest::Client::new().get(format!("{}/blockheight", ethereum_monitor_api)).send().await?;
    Ok(response.json::<BlockHeight>().await?.height)
}

async fn finish_block(ethereum_monitor_api: &String, block_height: u64) -> Result<(), reqwest::Error> {
    reqwest::Client::new().post(format!("{}/completeblock/{}", ethereum_monitor_api, block_height)).send().await?;
    Ok(())
}

#[derive(Deserialize)]
enum DepositResult {
    ok(serde_json::Value),
    err(serde_json::Value),
}

async fn deposit(ethereum_monitor_api: &String, deposit: &Deposit) -> Result<String, reqwest::Error> {
    reqwest::Client::new().post(format!("{}/deposit", ethereum_monitor_api)).json(&deposit).send().await?.text().await
}

static ERC20_PREFIX: &'static [u8] = &[169, 5, 156, 187];
static ZERO: &str = "0x0000000000000000000000000000000000000000";

fn tx_to_deposit(tx: &Transaction, block_height: u64) -> Option<Deposit> {
    let hash = tx.hash;
    if hash == "0xdecf5a32a00bcc1bc4ef7017607f4b3f75c8ae9d22a18df035f0de1e312d4c4c".parse::<B256>().unwrap() {
        println!("got eth")
    }
    if hash == "0x738df13b43b8a443b2fa0539f4b9e4c7d147713eda443015db1a1ced2e7588f9".parse::<B256>().unwrap() {
        println!("got usdc")
    }

    if let Some(to) = tx.to {
        let input = tx.input.to_vec();
        // println!("{} {}", encode(input.clone()), encode(tx.hash));

        if input.len() >= 68 && &input[0..4] == ERC20_PREFIX {
            let mut receiver_bytes: [u8; 20] = [0; 20];
            receiver_bytes.copy_from_slice(&input[16..36]);
            let receiver = Address::from(&receiver_bytes);

            let mut amount_bytes: [u8; 32] = [0; 32];
            amount_bytes.copy_from_slice(&input[36..68]);
            let amount = U256::from_be_bytes(amount_bytes).to_base_be(10000000000000000000).collect::<Vec<_>>();
            Some(Deposit {
                address: receiver.to_string(),
                tx: tx.hash.to_string(),
                amount: *amount.first().unwrap_or(&0),
                token_address: to.to_string(),
                block_height,
            })
        } else {
            let amount = tx.value.to_base_be(10000000000000000000).collect::<Vec<_>>();
            if amount.len() == 0 {
                None
            } else {
                Some(Deposit {
                    address: to.to_string(),
                    tx: tx.hash.to_string(),
                    amount: *amount.first().unwrap(),
                    token_address: ZERO.parse().unwrap(),
                    block_height,
                })
            }
        }
    } else {
        None
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a key
    GenKey {
        /// Mnemonic seed phrase
        #[arg(short, long, required = true)]
        phrase: String,
    },
    /// Monitor the chain for deposits and submits them
    Monitor {
        /// RPC Provider URL
        #[arg(short, long, required = true)]
        rpc_url: String,
        /// Ethereum Monitor API URL
        #[arg(short, long, required = true)]
        ethereum_monitor_url: String,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn gen_key(phrase: String) {
    let mnemonic: Mnemonic<English> = Mnemonic::new_from_phrase(&phrase).unwrap();
    let seed = mnemonic.to_seed(None).unwrap();

    let priv_key = XPrv::new(&seed).unwrap();
    let encoded_key = priv_key.to_extended_key(Prefix::XPRV).to_string();
    println!("Private Key: {:?}", encoded_key);

    let xpriv: XPrv = XPrv::from_str(&encoded_key).unwrap();

    let derived_key = xpriv.derive_child(ChildNumber(1)).unwrap();
    let pubkey = derived_key.public_key();

    let mut hasher = Keccak256::new();
    hasher.update(pubkey.public_key().to_bytes());
    let result = hasher.finalize();

    // Take the last 20 bytes as the Ethereum address
    let mut address = [0u8; 20];
    address.copy_from_slice(&result[12..32]);
    println!("Address: {}", Address::from(address));
}

async fn monitor(rpc_url: String, ethereum_monitor_url: String) {
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

    let sub = provider.subscribe_blocks().await.unwrap();

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream();

    let mut last_block = get_last_block(&ethereum_monitor_url).await.unwrap();
    println!("Starting block {last_block}");

    loop {
        let block = stream.next().await.unwrap();
        let number = block.header.number.unwrap();
        let blocks = last_block..=number;
        for num in blocks {
            println!("Block: {num}");
            let by_number = provider.get_block_by_number(BlockNumberOrTag::Number(num), true).await.unwrap().unwrap();
            let txes = match by_number.transactions {
                BlockTransactions::Full(txes) => txes,
                BlockTransactions::Hashes(hashes) => futures::future::join_all(hashes.iter().map(|hash| async {
                    provider.get_transaction_by_hash(*hash).await.unwrap()
                }).collect::<Vec<_>>()).await,
                BlockTransactions::Uncle => {
                    vec![]
                }
            };

            for tx in txes {
                if let Some(dep) = tx_to_deposit(&tx, num) {
                    let result = deposit(&ethereum_monitor_url, &dep).await.unwrap();
                    match serde_json::from_str::<DepositResult>(&result) {
                        Ok(DepositResult::ok(deposit)) => println!("Got deposit {deposit}"),
                        Ok(DepositResult::err(_)) => (),
                        Err(_) => (),
                    }
                }
            }
            finish_block(&ethereum_monitor_url, num).await.unwrap();
        }
        last_block = number;
    }
}

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::GenKey { phrase } => gen_key(phrase),
        Commands::Monitor { rpc_url, ethereum_monitor_url } => monitor(rpc_url, ethereum_monitor_url).await,
    }
}
