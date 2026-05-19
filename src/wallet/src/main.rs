mod core;
use anyhow::Result;
use clap::{Parser, Subcommand};
use core::{Config, Core, FeeConfig, FeeType, Recipient};
use futures::SinkExt;
use kanal::bounded;
use lib::types::Transaction;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::time::{self, Duration};
use toml;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[arg(short, long, value_name = "ADDRESS")]
    node: Option<String>,
}

fn generate_dummy_config(path: PathBuf) -> Result<()> {
    let dummy_config = Config {
        my_keys: vec![],
        contacts: vec![
            Recipient {
                name: "Alice".to_string(),
                key: PathBuf::from("alice.pub.pem"),
            },
            Recipient {
                name: "Bob".to_string(),
                key: PathBuf::from("bob.pub.pem"),
            },
        ],
        default_node: "127.0.0.1:9000".to_string(),
        fee_config: FeeConfig {
            fee_type: FeeType::Percent,
            value: 0.1,
        },
    };
    let config_str = toml::to_string_pretty(&dummy_config)?;
    std::fs::write(&path, config_str)?;
    println!("Dummy config generated at : {}", path.display());
    Ok(())
}

async fn update_utxos(core: Arc<Core>) {
    let mut interval = time::interval(Duration::from_secs(20));
    loop {
        interval.tick().await;
        if let Err(e) = core.fetch_utxos().await {
            eprintln!("Failed to update utxos : {}", e);
        }
    }
}

async fn handle_transactions(rx: kanal::AsyncReceiver<Transaction>, core: Arc<Core>) {
    while let Ok(transaction) = rx.recv().await {
        if let Err(e) = core.send_transaction(transaction).await {
            eprintln!("Failed to send transaction : {}", e);
        }
    }
}
#[derive(Subcommand)]
enum Commands {
    GenerateConfig {
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
    },
}
fn main() {
    println!("Hello, world!");
}
