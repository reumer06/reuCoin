use anyhow::Result;
use argh::FromArgs;
use dashmap::DashMap;
use lib::types::Blockchain;
use static_init::dynamic;
use std::fmt::format;
use std::path::Path;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

mod handler;
mod util;

/// command line arguments for the node
#[derive(FromArgs)]
struct Args {
    /// port to listen on (default:9000).
    #[argh(option, default = "9000")]
    port: u16,
    /// path to the blockchain storage file
    #[argh(option, default = "String::from(\"./blockchain.cbor\")")]
    blockchain_file: String,
    /// list of initial seed nodes to connect to
    #[argh(positional)]
    nodes: Vec<String>,
}

#[dynamic]
pub static BLOCKCHAIN: RwLock<Blockchain> = RwLock::new(Blockchain::new());

// Node pool
#[dynamic]
pub static NODES: DashMap<String, TcpStream> = DashMap::new();
#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args: Args = argh::from_env();
    // Access the parsed arguments
    let port = args.port;
    let block_chain_file = args.blockchain_file;
    let nodes = args.nodes;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    // periodically cleanup the mempool and save the blockchain
    // Check if blockchain_file exits
    tokio::spawn(util::cleanup());
    tokio::spawn(util::save(block_chain_file.clone()));

    if Path::new(&block_chain_file).exists() {
        util::load_blockchain(&block_chain_file).await?;
    } else {
        println!("blockchain file does not exist!");
        util::populate_connections(&nodes).await?;
        println!("total amount of known nodes: {}", NODES.len());
        if nodes.is_empty() {
            println!("no initial nodes provided, starting as a seed node");
        } else {
            let (longest_name, longest_count) = util::find_longest_blockchain_node().await?;
            // request the blockchain from the node with the longest blockchain
            util::download_blockchain(&longest_name, longest_count).await?;
            println!("blockchain downloaded from {}", longest_name);
            // recalculate utxos
            {
                let mut blockchain = BLOCKCHAIN.write().await;
                blockchain.rebuild_utxos();
            }
            // try to adjust difficulty
            {
                let mut blockchain = BLOCKCHAIN.write().await;
                blockchain.try_adjust_target();
            }
        }
    }
    println!("Listening on {}", addr);
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handler::handle_connection(socket));
    }
    Ok(())
}
