use std::fmt::{ self, Debug, Formatter };
use super::hashable::*;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub nonce: u64,
    pub payload: String,
}

// Constructor
impl Block {
    pub fn new(index: u32, timestamp: u128, prev_block_hash: Vec<u8>, nonce: u64, payload: String) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0, 32],
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

// Debuggin print implementation for Block
impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block [{}]: at: {}, hash: {}, with payload: {}, nonce: {}.", &self.index, &self.timestamp, &hex::encode(&self.hash), &self.payload.len(), &self.nonce,
        )
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
        bytes.extend(self.payload.as_bytes());

        bytes
    }
} 

pub fn check_difficulty(hash: &Vec<u8>, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}