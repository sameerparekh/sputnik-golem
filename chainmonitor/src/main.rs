use std::env;
use std::env::VarError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::string::ToString;

use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::client::WsConnect,
};
use alloy::eips::BlockNumberOrTag;
use alloy::primitives::{Address, U256};
use alloy::primitives::private::derive_more::Display;
use alloy::rpc::types::eth::{BlockTransactions, Transaction};
use alloy::signers::wallet::coins_bip39::{English, Mnemonic, MnemonicError};
use alloy::transports::{RpcError, TransportErrorKind};
use bip32::{ChildNumber, Prefix, PublicKey, XPrv};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

use crate::MonitorError::ReqwestError;

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
        write!(
            f,
            "{}, {}, {}, {}, {}",
            self.address, self.tx, self.amount, self.token_address, self.block_height
        )
    }
}

async fn get_last_block(ethereum_monitor_api: &String) -> Result<u64, reqwest::Error> {
    let response = reqwest::Client::new()
        .get(format!("{}/blockheight", ethereum_monitor_api))
        .send()
        .await?;
    Ok(response.json::<BlockHeight>().await?.height)
}

async fn finish_block(
    ethereum_monitor_api: &String,
    block_height: u64,
) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .post(format!(
            "{}/completeblock/{}",
            ethereum_monitor_api, block_height
        ))
        .send()
        .await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum DepositResult {
    Ok(serde_json::Value),
    Err(DepositError),
}

#[derive(Deserialize, Debug, Display)]
#[serde(rename_all = "kebab-case")]
pub enum DepositError {
    WrongBlock(u64),
    TxSeen(String),
    AccountantError(serde_json::Value),
    UnknownAddress(String),
    InvalidAddress(String),
    TokenExists(String),
}

async fn deposit(
    ethereum_monitor_api: &String,
    deposit: &Deposit,
) -> Result<String, reqwest::Error> {
    reqwest::Client::new()
        .post(format!("{}/deposit", ethereum_monitor_api))
        .json(&deposit)
        .send()
        .await?
        .text()
        .await
}

static ERC20_PREFIX: &[u8] = &[169, 5, 156, 187];
static ZERO: Lazy<Address> = Lazy::new(|| Address::from_slice(&[0u8; 20]));

fn tx_to_deposit(tx: &Transaction, block_height: u64) -> Option<Deposit> {
    if let Some(to) = tx.to {
        let input = tx.input.to_vec();
        // println!("{} {}", encode(input.clone()), encode(tx.hash));

        if input.len() >= 68 && &input[0..4] == ERC20_PREFIX {
            let mut receiver_bytes: [u8; 20] = [0; 20];
            receiver_bytes.copy_from_slice(&input[16..36]);
            let receiver = Address::from(&receiver_bytes);

            let mut amount_bytes: [u8; 32] = [0; 32];
            amount_bytes.copy_from_slice(&input[36..68]);
            let amount = U256::from_be_bytes(amount_bytes)
                .to_base_be(10000000000000000000)
                .collect::<Vec<_>>();
            Some(Deposit {
                address: receiver.to_string(),
                tx: tx.hash.to_string(),
                amount: *amount.first().unwrap_or(&0),
                token_address: to.to_string(),
                block_height,
            })
        } else {
            let amount = tx
                .value
                .to_base_be(10000000000000000000)
                .collect::<Vec<_>>();
            Some(Deposit {
                address: to.to_string(),
                tx: tx.hash.to_string(),
                amount: *amount.first().unwrap_or(&0),
                token_address: ZERO.to_string(),
                block_height,
            })
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

#[derive(Display)]
enum GenKeyError {
    MnemonicError(MnemonicError),
    Bip32Error(bip32::Error),
}

impl From<MnemonicError> for GenKeyError {
    fn from(value: MnemonicError) -> Self {
        GenKeyError::MnemonicError(value)
    }
}

impl From<bip32::Error> for GenKeyError {
    fn from(value: bip32::Error) -> Self {
        GenKeyError::Bip32Error(value)
    }
}

fn gen_key(phrase: String) -> Result<(), GenKeyError> {
    let mnemonic: Mnemonic<English> = Mnemonic::new_from_phrase(&phrase)?;
    let seed = mnemonic.to_seed(None)?;

    let private_key = XPrv::new(seed)?;
    let encoded_key = private_key.to_extended_key(Prefix::XPRV).to_string();
    println!("Private Key: {:?}", encoded_key);

    let xpriv: XPrv = XPrv::from_str(&encoded_key)?;

    let derived_key = xpriv.derive_child(ChildNumber(1))?;
    let pubkey = derived_key.public_key();

    let mut hasher = Keccak256::new();
    hasher.update(pubkey.public_key().to_bytes());
    let result = hasher.finalize();

    // Take the last 20 bytes as the Ethereum address
    let mut address = [0u8; 20];
    address.copy_from_slice(&result[12..32]);
    println!("Address: {}", Address::from(address));
    Ok(())
}

#[derive(Display)]
enum MonitorError {
    WrongBlock,
    InternalError(String),
    ReqwestError(reqwest::Error),
    EnvError(env::VarError),
}

impl From<dotenv::Error> for MonitorError {
    fn from(value: dotenv::Error) -> Self {
        MonitorError::InternalError(format!("{}", value))
    }
}

impl From<reqwest::Error> for MonitorError {
    fn from(value: reqwest::Error) -> Self {
        ReqwestError(value)
    }
}

impl From<VarError> for MonitorError {
    fn from(value: VarError) -> Self {
        MonitorError::EnvError(value)
    }
}

impl From<RpcError<TransportErrorKind>> for MonitorError {
    fn from(value: RpcError<TransportErrorKind>) -> Self {
        MonitorError::InternalError(format!("{}", value))
    }
}

async fn monitor(rpc_url: String, ethereum_monitor_url: String) -> Result<(), MonitorError> {
    dotenv()?;
    let api_key = env::var("ALCHEMY_API_KEY")?;
    let ws = WsConnect::new(format!("{}/{}", rpc_url, api_key));
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    let sub = provider.subscribe_blocks().await?;

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream();

    let mut last_block = get_last_block(&ethereum_monitor_url).await?;
    println!("Starting block {last_block}");

    loop {
        let block = match stream.next().await {
            Some(block) => block,
            None => {
                println!("No block received!");
                continue;
            }
        };
        let number = match block.header.number {
            Some(number) => number,
            None => {
                println!("Block has no number!");
                continue;
            }
        };
        let blocks = last_block..=number;
        for num in blocks {
            println!("Block: {num}");
            if let Some(by_number) = provider
                .get_block_by_number(BlockNumberOrTag::Number(num), true)
                .await? {
                let txes = match by_number.transactions {
                    BlockTransactions::Full(txes) => txes,
                    BlockTransactions::Hashes(hashes) => {
                        futures::future::join_all(
                            hashes
                                .iter()
                                .map(|hash| async {
                                    match provider.get_transaction_by_hash(*hash).await {
                                        Ok(tx) => Some(tx),
                                        Err(_) => None
                                    }
                                })
                                .collect::<Vec<_>>(),
                        ).await.into_iter().flatten().collect::<Vec<_>>()
                    }
                    BlockTransactions::Uncle => {
                        vec![]
                    }
                };

                for tx in txes {
                    if let Some(dep) = tx_to_deposit(&tx, num) {
                        let result = deposit(&ethereum_monitor_url, &dep).await?;
                        match serde_json::from_str::<DepositResult>(&result) {
                            Ok(DepositResult::Ok(deposit)) => println!("Deposit: {deposit}"),
                            Ok(DepositResult::Err(DepositError::WrongBlock(expected_block))) => {
                                println!("Wrong block: {num} Expecting: {expected_block}");
                                return Err(MonitorError::WrongBlock);
                            }
                            Ok(DepositResult::Err(_)) => (), // Ignore other failed deposits
                            Err(err) => println!("Error: {err}"),
                        }
                    }
                }
                finish_block(&ethereum_monitor_url, num).await?;
            }
        }

        last_block = number;
    }
}

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::GenKey { phrase } => gen_key(phrase).unwrap_or_else(|err| panic!("GenKey failed: {}", err)),
        Commands::Monitor {
            rpc_url,
            ethereum_monitor_url,
        } => monitor(rpc_url, ethereum_monitor_url)
            .await
            .unwrap_or_else(|err| panic!("Monitor failed: {}", err)),
    }
}
