use chrono::Utc;
use lib::network::Message;
use lib::sha256::Hash;
use lib::types::{Block, BlockHeader, Transaction, TransactionOutput};
use lib::util::MerkleRoot;
use tokio::net::TcpStream;
// use tokio::net::windows::named_pipe::PipeMode::Message;
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
    }
}
