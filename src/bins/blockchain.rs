use super::block::Block;
use super::hashable::Hashable;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&self) -> Result<(), &str> {
        for (index, block) in self.blocks.iter().enumerate() {
            // Previous block
            let previous_block = &self.blocks[index - 1];

            // Genesis block
            if index == 0 { return Ok(()) }
            // Check index
            if index as u32 != block.index { return Err("Block index is not correct") };
            // Check difficulty
            if !block.check_difficulty(&block.hash(), block.difficulty) { return Err("Difficulty fail") }
            // Check time
            if block.timestamp <= previous_block.timestamp { return Err("Time did not increase between blocks")};
            // Check hash
            if block.prev_block_hash != previous_block.hash { return Err("Current's block previous hash and previous block hash don't match")}
        }

        return Ok(());
    }
}