use crate::U256;
use crate::error::ReuError;
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{Transaction,TransactionOutput}

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
    pub fn verify_transactions(
        &self,
        predicted_block_height: u64,
        utxos: &HashMap<Hash, (bool, TransactionOutput)>,
    ) -> crate::error::Result<()> {
        let mut inputs: HashMap<Hash, TransactionOutput> = HashMap::new();
        // reject complete empty blocks
        if self.transactions.is_empty() {
            return Err(ReuError::InvalidTransaction);
        }
        //  verify coinbase transaction -> the first transaction in a block is called as coinbase transaction
        self.verify_coinbase_transaction(predicted_block_height, utxos)?;

        for transaction in self.transactions.iter().skip(1) {
            let mut input_value = 0;
            let mut output_value = 0;
            for input in &transaction.inputs {
                let prev_output = utxos
                    .get(&input.prev_transaction_output_hash)
                    .map(|(_, output)| output);
                if prev_output.is_none() {
                    return Err(ReuError::InvalidTransaction);
                }
                let prev_output = prev_output.unwrap();
                // prev same block double spending
                if inputs.contains_key(&input.prev_transaction_output_hash) {
                    return Err(ReuError::InvalidTransaction);
                }
                // check if signature is valid
                if !input
                    .signature
                    .verify(&input.prev_transaction_output_hash, &prev_output.pubkey)
                {
                    return Err(ReuError::InvalidSignature);
                }
                input_value += prev_output.value;
                inputs.insert(input.prev_transaction_output_hash, prev_output.clone());
            }
            for output in &transaction.outputs {
                output_value += output.value;
            }
            // difference is the fee for miner
            if input_value < output_value {
                return Err(ReuError::InvalidTransaction);
            }
        }
        Ok(())
    }

    pub fn verify_coinbase_transaction(
        &self,
        predicted_block_height: u64,
        utxos: &HashMap<Hash, (bool, TransactionOutput)>,
    ) -> crate::error::Result<()> {
        let coinbase_tran = &self.transactions[0];
        if coinbase_tran.inputs.len() != 0 {
            return Err(ReuError::InvalidTransaction);
        }
        if coinbase_tran.outputs.len() == 0 {
            return Err(ReuError::InvalidTransaction);
        }
        let miner_fess = self.calculate_miner_fess(utxos)?;
        let block_reward = crate::INITIAL_REWARD * 10u64.pow(8)
            / 2u64.pow((predicted_block_height / crate::HALVING_INTERVAL) as u32);
        let total_coinbase_outputs: u64 = coinbase_tran
            .outputs
            .iter()
            .map(|output| output.value)
            .sum();
        if total_coinbase_outputs != block_reward + miner_fess {
            return Err(ReuError::InvalidTransaction);
        }
        Ok(())
    }   

    pub fn calculate_miner_fess(
        &self,
        utxos: &HashMap<Hash, (bool, TransactionOutput)>,
    ) -> crate::error::Result<u64> {
        let mut inputs: HashMap<Hash, TransactionOutput> = HashMap::new();
        let mut outputs: HashMap<Hash, TransactionOutput> = HashMap::new();
        for transactions in self.transactions.iter().skip(1) {
            for input in &transactions.inputs {
                let prev_outputs = utxos
                    .get(&input.prev_transaction_output_hash)
                    .map(|(_, outputs)| outputs);
                if prev_outputs.is_none() {
                    return Err(ReuError::InvalidTransaction);
                }
                let prev_output = prev_outputs.unwrap();
                if inputs.contains_key(&input.prev_transaction_output_hash) {
                    return Err(ReuError::InvalidTransaction);
                }
                inputs.insert(input.prev_transaction_output_hash, prev_output.clone());
            }
            for output in &transactions.outputs {
                let output_hash = output.hash();
                if outputs.contains_key(&output_hash) {
                    return Err(ReuError::InvalidTransaction);
                }
                outputs.insert(output_hash, output.clone());
            }
        }
        let input_value: u64 = inputs.values().map(|output| output.value).sum();
        let outputs_value: u64 = outputs.values().map(|output| output.value).sum();

        if input_value < outputs_value {
            return Err(ReuError::InvalidTransaction);
        }

        Ok(input_value - outputs_value)
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

    pub fn mine(&mut self, steps: usize) -> bool {
        // if the hashes already matches the target, return early
        if self.hash().matches_target(self.target) {
            return true;
        }
        for _ in 0..steps {
            if let Some(new_nonce) = self.nonce.checked_add(1) {
                self.nonce = new_nonce;
            } else {
                self.nonce = 0;
                self.timestamp = Utc::now()
            }
            if self.hash().matches_target(self.target) {
                return true;
            }
        }
        false
    }
}
