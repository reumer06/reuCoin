use crate::U256;
use crate::crypto::{PublicKey, Signature};
use crate::error::{Result, ReuError};
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    utxos: HashMap<Hash, (bool, TransactionOutput)>,
    #[serde(default, skip_serializing)]
    mempool: Vec<(DateTime<Utc>, Transaction)>,
    target: U256,
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            utxos: HashMap::new(),
            mempool: vec![],
            blocks: vec![],
            target: crate::MIN_TARGET,
        }
    }
    pub fn utxos(&self) -> &HashMap<Hash, (bool, TransactionOutput)> {
        &self.utxos
    }

    pub fn target(&self) -> U256 {
        self.target
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }

    pub fn block_height(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn mempool(&self) -> &[(DateTime<Utc>, Transaction)] {
        &self.mempool
    }

    // add a transaction to mempool
    pub fn add_to_mempool(&mut self, transaction: Transaction) -> Result<()> {
        // validate transaction before insertion
        //all inputs must match known UTXOS, and must be unique
        let mut known_inputs = HashSet::new();
        for input in &transaction.inputs {
            // check if any of the utxos have the bool mark set to true
            // if so, find the transaction that references them in mempool, remove it, and set all the utxos it reference to false
            if !self.utxos.contains_key(&input.prev_transaction_output_hash) {
                return Err(ReuError::InvalidTransaction);
            }
            if known_inputs.contains(&input.prev_transaction_output_hash) {
                return Err(ReuError::InvalidTransaction);
            }
            known_inputs.insert(input.prev_transaction_output_hash);
        }
        for input in &transaction.inputs {
            if let Some((true, _)) = self.utxos.get(&input.prev_transaction_output_hash) {
                let referencing_transaction =
                    self.mempool
                        .iter()
                        .enumerate()
                        .find(|(_, (_, transaction))| {
                            transaction
                                .outputs
                                .iter()
                                .any(|output| output.hash() == input.prev_transaction_output_hash)
                        });

                // if we have found one, unmark all of its utxos
                if let Some((idx, (_, referencing_transaction))) = referencing_transaction {
                    for input in &referencing_transaction.inputs {
                        // set all utxos from this transaction to false
                        self.utxos
                            .entry(input.prev_transaction_output_hash)
                            .and_modify(|(marked, _)| {
                                *marked = false;
                            });
                    }
                    self.mempool.remove(idx);
                } else {
                    self.utxos
                        .entry(input.prev_transaction_output_hash)
                        .and_modify(|(marked, _)| {
                            *marked = false;
                        });
                }
            }
        }
        let all_inputs = transaction
            .inputs
            .iter()
            .map(|input| {
                self.utxos
                    .get(&input.prev_transaction_output_hash)
                    .expect("BUG: impossible")
                    .1
                    .value
            })
            .sum::<u64>();
        let all_outputs = transaction.outputs.iter().map(|output| output.value).sum();
        if all_inputs < all_outputs {
            return Err(ReuError::InvalidTransaction);
        }
        // Mark the UTXOs as used
        for input in &transaction.inputs {
            self.utxos
                .entry(input.prev_transaction_output_hash)
                .and_modify(|(marked, _)| {
                    *marked = true;
                });
        }

        // push the transaction to the mempool
        self.mempool.push((Utc::now(), transaction));
        // sort by miner fee
        self.mempool.sort_by_key(|(_, transaction)| {
            // sum total value available from previous outputs
            let all_inputs = transaction
                .inputs
                .iter()
                .map(|input| {
                    self.utxos
                        .get(&input.prev_transaction_output_hash)
                        .expect("BUG: Impossible")
                        .1
                        .value
                })
                .sum::<u64>();
            // sum value sent to new recipients
            let all_outputs: u64 = transaction.outputs.iter().map(|output| output.value).sum();
            // implicit fee for the miner
            let miner_fee = all_inputs - all_outputs;
            miner_fee
        });
        Ok(())
    }

    pub fn clean_mempool(&mut self) {
        let now = Utc::now();
        let max_age = chrono::Duration::seconds(crate::MAX_MEMPOOL_TRANSACTION_AGE as i64);
        let mut utxos_hashes_to_unmark: Vec<Hash> = vec![];
        self.mempool.retain(|(timestamp, transaction)| {
            if now - *timestamp > max_age {
                // push all utxos to unmark the vector to unmark it later
                utxos_hashes_to_unmark.extend(
                    transaction
                        .inputs
                        .iter()
                        .map(|input| input.prev_transaction_output_hash),
                );
                false // remove from mempool
            } else {
                true // keep in mempool
            }
        });
        // unmark the UTXOs
        for hash in utxos_hashes_to_unmark {
            self.utxos.entry(hash).and_modify(|(marked, _)| {
                *marked = false;
            });
        }
    }
    pub fn add_blocks(&mut self, block: Block) -> Result<()> {
        if self.blocks.is_empty() {
            if block.header.prev_block_hash != Hash::zero() {
                println!("zero hash");
                return Err(ReuError::InvalidBlock);
            }
        } else {
            let last_block = self.blocks.last().unwrap();
            if block.header.prev_block_hash != last_block.hash() {
                println!("prev hash is wrong");
                return Err(ReuError::InvalidBlock);
            }
            // check if the block timestamp is after the last block's timestamp
            if block.header.timestamp <= last_block.header.timestamp {
                return Err(ReuError::InvalidBlock);
            }
        }

        let block_transactions: HashSet<_> =
            block.transactions.iter().map(|tx| tx.hash()).collect();

        self.mempool
            .retain(|tx| !block_transactions.contains(&tx.1.hash()));

        // check if the block's hash is less than target
        if !block.header.hash().matches_target(block.header.target) {
            println!("does not match the target");
            return Err(ReuError::InvalidBlock);
        }
        let calculated_merkle_root = MerkleRoot::calculate(&block.transactions);
        if calculated_merkle_root != block.header.merkle_root {
            println!("invalid merkle root");
            return Err(ReuError::InvalidMerkleRoot);
        }

        // verify all transactions in the block at the next height
        block.verify_transactions(self.blocks.len() as u64, &self.utxos)?;

        self.blocks.push(block);
        self.try_adjust_target();
        self.rebuild_utxos(); // keep state synced
        Ok(())
    }

    // try to adjust the target of the blockchain
    pub fn try_adjust_target(&mut self) {
        if self.blocks.is_empty() {
            return;
        }
        if self.blocks.len() % crate::DIFFICULTY_UPDATE_INTERVAL as usize != 0 {
            return;
        }
        // measure the time it took to mine the last block
        let start_time = self.blocks
            [self.blocks.len() - crate::DIFFICULTY_UPDATE_INTERVAL as usize]
            .header
            .timestamp;
        let end_time = self.blocks.last().unwrap().header.timestamp;
        let time_diff = end_time - start_time;
        // comver time_diff to seconds
        let time_diff_seconds = time_diff.num_seconds();
        // calculate the ideal number of seconds
        let target_seconds = crate::IDEAL_BLOCK_TIME * crate::DIFFICULTY_UPDATE_INTERVAL;
        // multiply the current target by actual time divided by ideal time
        let new_target = BigDecimal::parse_bytes(&self.target.to_string().as_bytes(), 10)
            .expect("BUG: impossible")
            * (BigDecimal::from(time_diff_seconds) / BigDecimal::from(target_seconds));
        // cut off decimal point and everything after it from the string representation of new_target
        let new_target_str = new_target
            .to_string()
            .split('.')
            .next()
            .expect("BUG: Expected a Decimal point")
            .to_owned();
        let new_target: U256 = U256::from_str_radix(&new_target_str, 10).expect("BUG: impossible");
        // clamp the new target to be within the range (factor of 4 clamp)
        let new_target = if new_target < self.target / 4 {
            self.target / 4
        } else if new_target > self.target * 4 {
            self.target * 4
        } else {
            new_target
        };
        // if the target is more than the minimum target set it to minimum target
        self.target = new_target.min(crate::MIN_TARGET);
    }
    pub fn rebuild_utxos(&mut self) {
        self.utxos.clear(); // chain state and utxos stays in sync

        for blocks in &self.blocks {
            for transaction in &blocks.transactions {
                // spend referenced outputs
                for input in &transaction.inputs {
                    self.utxos.remove(&input.prev_transaction_output_hash);
                }
                // add newly in created outputs
                for output in &transaction.outputs {
                    self.utxos.insert(output.hash(), (false, output.clone()));
                }
            }
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
    pub fn verify_transactions(
        &self,
        predicted_block_height: u64,
        utxos: &HashMap<Hash, (bool, TransactionOutput)>,
    ) -> Result<()> {
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
    ) -> Result<()> {
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
    ) -> Result<u64> {
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
