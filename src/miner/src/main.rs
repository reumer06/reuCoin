use anyhow::{Result, anyhow};
use clap::Parser;
use lib::crypto::PublicKey;
use lib::network::Message;
use lib::types::Block;
use lib::util::Saveable;
use std::env;
use std::process::exit;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{Duration, interval};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    address: String,

    #[arg(short, long)]
    public_key_file: String,
}
struct Miner {
    public_key: PublicKey,
    stream: Mutex<TcpStream>,
    current_template: Arc<std::sync::Mutex<Option<Block>>>,
    mining: Arc<AtomicBool>,
    mined_block_sender: flume::Sender<Block>,
    miner_block_receiver: flume::Receiver<Block>,
}

impl Miner {
    async fn new(address: String, public_key: PublicKey) -> Result<Self> {}
    async fn run(&self) -> Result<()> {}
    fn spawn_mining_thread(&self) -> thread::JoinHandle<()> {}
    async fn fetch_and_validate_template(&self) -> Result<()> {}
    async fn fetch_template(&self) -> Result<()> {}
    async fn validate_template(&self) -> Result<()> {}
    async fn submit_block(&self, block: Block) -> Result<()> {}
}
// fn usage() -> ! {
//     eprintln!(
//         "Usage: {} <address> <public_key_file>",
//         env::args().next().unwrap()
//     );
//     exit(1);
// }

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let public_key = PublicKey::load_from_file(&cli.public_key_file)
        .map_err(|e| anyhow!("Error reading public key: {}", e))?;
    let miner = Miner::new(cli.address, public_key).await?;
    miner.run().await
}
