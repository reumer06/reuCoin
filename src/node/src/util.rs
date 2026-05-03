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
    let mut blockchain = crate::BLOCKCHAIN.write().await;
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

pub async fn populate_connections(nodes: &[String]) -> Result<()> {
    println!("trying to connect to other nodes...");
    for node in nodes {
        println!("connecting to {}", node);
        let mut stream = TcpStream::connect(&node).await?;
        let message = Message::DiscoverNodes;
        message.send_async(&mut stream).await?;
        println!("sent DiscoverNodes to {}", node);
        let message = Message::receive_async(&mut stream).await?;
        match message {
            Message::NodeList(child_nodes) => {
                println!("received NodeList from {}", node);
                for child_node in child_nodes {
                    println!("adding node {}", child_node);
                    let new_stream = TcpStream::connect(&child_node).await?;
                    crate::NODES.insert(child_node, new_stream);
                }
            }
            _ => {
                println!("unexpected message from {}", node);
            }
        }
        crate::NODES.insert(node.clone(), stream);
    }
    Ok(())
}
pub async fn find_longest_blockchain_node() {}
pub async fn download_blockchain() {}
