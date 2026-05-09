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
            FetchBlock(height) => {
                let blockchain = crate::BLOCKCHAIN.read().await;
                let Some(block) = blockchain.blocks().nth(height as usize).cloned() else {
                    return;
                };
                let message = NewBlock(block);
                message.send_async(&mut socket).await.unwrap();
            }
            DiscoverNodes => {
                let nodes = crate::NODES
                    .iter()
                    .map(|x| x.key().clone())
                    .collect::<Vec<_>>();
                let message = NodeList(nodes);
                message.send_async(&mut socket).await.unwrap();
            }
            AskDifference(height) => {
                let blockchain = crate::BLOCKCHAIN.read().await;
                let count = blockchain.block_height() as i32 - height as i32;
                let message = Difference(count);
                message.send_async(&mut socket).await.unwrap();
            }
            FetchUTXOs(key) => {
                println!("received  request to fetch UTXOs");
                let blockchain = crate::BLOCKCHAIN.read().await;
                let utxos = blockchain
                    .utxos()
                    .iter()
                    .filter(|(_, (_, txout))| txout.pubkey == key)
                    .map(|(_, (marked, txout))| (txout.clone(), *marked))
                    .collect::<Vec<_>>();
                let message = UTXOs(utxos);
                message.send_async(&mut socket).await.unwrap();
            }
            NewBlock(block) => {
                let mut blockchain = crate::BLOCKCHAIN.write().await;
                println!("received new block");
                if blockchain.add_blocks(block).is_err() {
                    println!("block rejected");
                }
            }
            NewTransaction(tx) => {
                let mut blockchain = crate::BLOCKCHAIN.write().await;
                println!("received transaction from friend");
                if blockchain.add_to_mempool(tx).is_err() {
                    println!("transaction rejected, closing connection");
                    return;
                }
            }
            ValidateTemplate(block_template) => {}
            SubmitTemplate(block) => {}
            SubmitTransaction(tx) => {}
            FetchTemplate(pubkey) => {}
        }
    }
}
