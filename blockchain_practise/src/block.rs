use sha2::{Digest, Sha256}; // Hashing library
use std::time::{SystemTime, UNIX_EPOCH};
pub use serde_derive::{Serialize, Deserialize};


#[derive(serde_derive::Serialize, Deserialize)]
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub previous_hash: String,
    pub data: String,
    pub hash: String,
    pub nonce: u64 // Used for Proof of Work
}

impl Block {
    // Creating new block
    pub fn new(index: usize, 
    previous_hash: String,
    data_input: String) -> Self {
        let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backawards")
        .as_secs();
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            data: data_input,
            hash: String::new(),
            nonce: 0
        };
        block.mine_block(2); // POW with 2 leading zeros?
        block // returning the block
    }

    //create a hash of the block
    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        let encoded = hasher.finalize();
        hex::encode(encoded)
    }

    // Proof of Work: Find a hash with `difficulty` leading zeros
    pub fn mine_block(&mut self, difficulty: usize) {
        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with(&"0".repeat(difficulty)) {
                break;
            }
            self.nonce += 1;
        }
        println!(
            "Block {} mined! Previous Hash: {} Hash: {} | Nonce: {}",
            self.index, self.previous_hash, self.hash, self.nonce
        );
    }
}
