mod core;
use anyhow::Result;
use clap::{Parser, Subcommand};
use kanal::bounded;
use lib::types::Transaction;
use std::io::{self, Write};
use std::path::PathBuf;
use tokio::time::{self, Duration};

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
