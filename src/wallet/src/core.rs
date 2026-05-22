use anyhow::Result;
use crossbeam_skiplist::SkipMap;
use kanal::AsyncSender;
use lib::crypto::{PrivateKey, PublicKey};
use lib::network::Message;
use lib::types::{Transaction, TransactionOutput};
use lib::util::Saveable;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::net::windows::named_pipe::PipeMode::Message;

#[derive(Serialize, Deserialize, Clone)]
pub struct Key {
    public: PathBuf,
    private: PathBuf,
}

#[derive(Clone)]
struct LoadedKey {
    public: PublicKey,
    private: PrivateKey,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Recipient {
    pub name: String,
    pub key: PathBuf,
}

#[derive(Clone)]
pub struct LoadedRecipient {
    pub name: String,
    pub key: PublicKey,
}

impl Recipient {
    pub fn load(&self) -> Result<LoadedRecipient> {
        let key = PublicKey::load_from_file(&self.key)?;
        Ok(LoadedRecipient {
            name: self.name.clone(),

            key,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum FeeType {
    Fixed,
    Percent,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct FeeConfig {
    pub fee_type: FeeType,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub my_keys: Vec<Key>,
    pub contacts: Vec<Recipient>,
    pub default_node: String,
    pub fee_config: FeeConfig,
}

#[derive(Clone)]
struct UtxoStore {
    my_keys: Vec<LoadedKey>,
    utxos: Arc<SkipMap<PublicKey, Vec<(bool, TransactionOutput)>>>,
}

impl UtxoStore {
    fn new() -> Self {
        UtxoStore {
            my_keys: Vec::new(),
            utxos: Arc::new(SkipMap::new()),
        }
    }
    fn add_key(&mut self, key: LoadedKey) {
        self.my_keys.push(key)
    }
}

pub struct Core {
    pub config: Config,
    utxos: UtxoStore,
    pub tx_sender: AsyncSender<Transaction>,
}

impl Core {
    fn new(config: Config, utxo_store: UtxoStore) -> Self {
        let (tx_sender, _) = kanal::bounded(10);
        Core {
            config,
            utxos,
            tx_sender: tx_sender.clone_async(),
        }
    }
    pub fn load(config_path: PathBuf) -> Result<Self> {
        let config: Config = toml::from_str(&fs::read_to_string(&config_path)?)?;
        let mut utxos = UtxoStore::new();
        // Load keys from config
        for key in &config.my_keys {
            let mut public = PublicKey::load_from_file(&key.public)?;
            let private = PrivateKey::load_from_file(&key.private)?;
            utxos.add_key(LoadedKey { public, private });
        }
        Ok(Core::new(config, utxos))
    }
    pub async fn fetch_utxos(&self) -> Result<()> {
        let mut stream = TcpStream::connect(&self.config.default_node).await?;
        for key in &self.utxos.my_keys {
            let message = Message::FetchUTXOs(key.public.clone());
            message.send_async(&mut stream).await?;
            if let Message::UTXOs(utxos) = Message::receive_async(&mut stream).await? {
                self.utxos.utxos.insert(
                    key.public.clone(),
                    utxos
                        .into_iter()
                        .map(|(output, marked)| (marked, output))
                        .collect(),
                );
            } else {
                return Err(anyhow::anyhow!("Unexpected response from the node"));
            }
        }
        Ok(())
    }
    pub async fn send_transaction(&self, transaction: Transaction) -> Result<()> {
        Ok(())
    }
    pub fn get_balance(&self) -> u64 {}
    pub async fn create_transaction(
        &self,
        recipient: &PublicKey,
        amount: u64,
    ) -> Result<Transaction> {
    }
    fn calculate_fee(&self, amount: u64) -> u64 {} {
}
