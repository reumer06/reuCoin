use chrono::Utc;
use lib::network::Message;
use lib::sha256::Hash;
use lib::types::{Block, BlockHeader, Transaction, TransactionOutput};
use lib::util::MerkleRoot;
use tokio::net::TcpStream;
use uuid::Uuid;

pub async fn handle_connection(mut socket: TcpStream) {
    loop {
        // read a message from the socket
        let message = match Message::receive_async(&mut socket).await {
            Ok(message) => message,
            Err(e) => {
                println!("invalid message from peer:{e}, closing that connection");
                return;
            }
        };
        use lib::network::Message::*;
        match message {
            UTXOs(_) | Template(_) | Difference(_) | TemplateValidity(_) | NodeList(_) => {
                println!("neither a miner nor a wallet! Goodbye");
                return;
            }
            FetchBlock(height) => {}
            DiscoverNodes => {}
            AskDifference(height) => {}
            FetchUTXOs(key) => {}
            NewBlock(block) => {}
            NewTransaction(tx) => {}
            ValidateTemplate(block_template) => {}
            SubmitTemplate(block) => {}
            SubmitTransaction(tx) => {}
            FetchTemplate(pubkey) => {}
        }
    }
}
