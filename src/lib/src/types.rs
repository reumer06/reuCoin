use crate::U256;
use crate::crypto::{PublicKey, Signature};
use crate::error::{Result, ReuError};
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }
    pub fn add_blocks(&mut self, block: Block) -> Result<()> {
        if self.blocks.is_empty() {
            if block.header.prev_block_hash != Hash::zero() {
                println!("zero hash");
                return Err(ReuError::InvalidBlock);
            } else {
                let last_block = self.blocks.last().unwrap();
                if block.header.prev_block_hash != last_block.hash() {
                    println!("prev hash is wrong");
                    return Err(ReuError::InvalidBlock);
                }
                if !block.header.hash().matches_target(block.header.target) {
                    println!("does not match the target");
                    return Err(ReuError::InvalidBlock);
                }
                let calculated_merkle_root = MerkleRoot::calculate(&block.transactions);
                if calculated_merkle_root != block.header.merkle_root {
                    println!("invalid merkle root");
                    return Err(ReuError::InvalidMerkleRoot);
                }
                if block.header.timestamp <= last_block.header.timestamp {
                    return Err(ReuError::InvalidBlock);
                }
                block.verify_transactions(self.blocks_heights(), &self.utxos)?;
            }
            self.blocks.push(block);
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header,
            transactions,
        }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    // timestamp of the block.
    pub timestamp: DateTime<Utc>,
    // Nonce used to mine the block.
    pub nonce: u64,
    // hash of the previous block.
    pub prev_block_hash: Hash,
    // merkle root of the block's transactions.
    pub merkle_root: MerkleRoot,
    // target.
    pub target: U256, // has to higher than hash of this block to be considered valid.
}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256,
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_key: Uuid,
    pub pubkey: PublicKey,
}

impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction { inputs, outputs }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}
