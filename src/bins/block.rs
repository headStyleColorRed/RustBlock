use std::fmt::{ self, Debug, Formatter };
use super::hashable::*;
use super::transaction::Transaction;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

// Constructor
impl Block {
    pub fn new(index: u32, timestamp: u128, prev_block_hash: Vec<u8>, nonce: u64, transactions: Vec<Transaction>, difficulty: u128) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0, 32],
            prev_block_hash,
            nonce,
            transactions,
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        for nonce_attemp in 0..(u64::max_value()) {
            self.nonce = nonce_attemp;
            let hash = self.hash();
            if self.check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }

    pub fn check_difficulty(&self, hash: &Vec<u8>, difficulty: u128) -> bool {
        difficulty > difficulty_bytes_as_u128(&hash)
    }
}

// Debuggin print implementation for Block
impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block [{}]: at: {}, hash: {}, with transactions: {}, nonce: {}.", &self.index, &self.timestamp, &hex::encode(&self.hash), &self.transactions.len(), &self.nonce)
    }
}

// Hashes
impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes =  vec![];
        bytes.extend(u32_bytes(&self.index));
        bytes.extend(u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(u64_bytes(&self.nonce));
        bytes.extend(self.transactions.iter().flat_map(|transaction| transaction.bytes()).collect::<Vec<u8>>());
        bytes.extend(u128_bytes(&self.difficulty));

        bytes
    }
} 
