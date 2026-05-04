use chrono::Utc;
use lib::network::Message;
use lib::sha256::Hash;
use lib::types::{Block, BlockHeader, Transaction, TransactionOutput};
use lib::util::MerkleRoot;
use tokio::net::TcpStream;
use uuid::Uuid;

pub async fn handle_connection(mut socket: TcpStream) {}
