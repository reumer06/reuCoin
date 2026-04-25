use lib::crypto::PublicKey;
use lib::util::Saveable;
use std::env;
use std::process::exit;
use tokio::net::TcpStream;

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
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", address, e);
            exit(1);
        }
    };
}
