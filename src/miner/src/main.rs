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

fn usage() -> ! {
    eprintln!(
        "Usage: {} <address> <public_key_file>",
        env::args().next().unwrap()
    );
    exit(1);
}

#[tokio::main]
async fn main() {
    let address = match env::args().nth(1) {
        Some(address) => address,
        None => usage(),
    };

    let public_key_file = match env::args().nth(2) {
        Some(public_key_file) => public_key_file,
        None => usage(),
    };

    let Ok(PublicKey) = PublicKey::load_from_file(&public_key_file) else {
        eprintln!("Error reading public key from file {} ", public_key_file);
        exit(1);
    };
    println!("Connecting to {address} to mine with {PublicKey:?}");
    let mut stream = match TcpStream::connect(&address).await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", address, e);
            exit(1);
        }
    };
    let message = Message::FetchTemplate(PublicKey);
    message.send(&mut stream);
}
