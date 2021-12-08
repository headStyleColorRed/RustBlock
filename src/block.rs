use std::fmt::{ self, Debug, Formatter };
use super::*;

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