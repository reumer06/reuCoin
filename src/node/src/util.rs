use anyhow::{Context, Result};
use lib::network::Message;
use lib::types::Blockchain;
use lib::util::Saveable;
use tokio::net::TcpStream;
use tokio::time;

pub async fn load_blockchain(blockchain_file: &str) -> Result<()> {
    println!("blockchain exits, file loading...");
    let new_blockchain = Blockchain::load_from_file(blockchain_file)?;
    println!("Blockchain loaded");
    let mut blockchain = crate::BLOCKCHAIN.write().await?;
    *blockchain = new_blockchain;
    println!("rebuilding utxos...");
    blockchain.rebuild_utxos();
    println!("utxos rebuilt");
    println!("checking if target needs to be adjusted...");
    println!("current target : {}", blockchain.target());
    blockchain.try_adjust_target();
    println!("new target: {}", blockchain.target());
    println!("initialization complete");
    Ok(())
}
