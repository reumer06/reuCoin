use anyhow::Result;
use argh::FromArgs;
use dashmap::DashMap;
use lib::types::Blockchain;
use static_init::dynamic;
use std::path::Path;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

mod handler;
mod util;
#[derive(FromArgs)]
struct Args {
    #[args(option, default = "9000")]
    port: u16,
    #[args(option, default = "String::from(\"./blockchain.cbor\")")]
    blockchain_file: String,
    #[argh(positional)]
    nodes: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let port = args.port;
    let block_chain_file = args.blockchain_file;
    let nodes = args.nodes;
    Ok(())
}
