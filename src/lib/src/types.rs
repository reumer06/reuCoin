use crate::U256;
use uuid::Uuid;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }
    pub fn add_blocks(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
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
    pub fn hash(&self) -> ! {
        unimplemented!() // never type.
    }
}

pub struct BlockHeader {
    // timestamp of the block.
    pub timestamp: u64,
    // Nonce used to mine the block.
    pub nonce: u64,
    // hash of the previous block.
    pub prev_block_hash: [u8; 32],
    // merkle root of the block's transactions.
    pub merkle_root: [u8; 32],
    // target.
    pub target: U256, // has to higher than hash of this block to be considered valid.
}

impl BlockHeader {
    pub fn new(
        timestamp: u64,
        nonce: u64,
        prev_block_hash: [u8; 32],
        merkle_root: [u8; 32],
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
    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}
pub struct TransactionInput {
    pub prev_transaction_output_hash: [u8; 32],
    pub signature: [u8; 64],
}
pub struct TransactionOutput {
    pub value: u64,
    pub unique_key: Uuid,
    pub pubkey: [u8; 33],
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction { inputs, outputs }
    }
    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}
