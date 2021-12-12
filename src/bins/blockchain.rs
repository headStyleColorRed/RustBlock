use super::block::Block;
use super::blockValidationErr::BlockValidationErr;
use super::hashable::Hashable;
use std::collections::HashSet;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub unspent_outputs: HashSet<Vec<u8>>
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn add_genesis_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        // Check index
        if block.index != 0 { return Err(BlockValidationErr::MismatchedIndex) };
        // Check difficulty
        if !block.check_difficulty(&block.hash(), block.difficulty) { return Err(BlockValidationErr::InvalidHash) }

        // T R A N S A C T I O N S
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() { return Err(BlockValidationErr::InvalidCoinbaseTransaction) };

            let mut block_spent: HashSet<Vec<u8>> = HashSet::new();
            let mut block_created: HashSet<Vec<u8>> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                
                // No double spending
                if !(&input_hashes - &self.unspent_outputs).is_empty() || !(&input_hashes & &block_spent).is_empty() { return Err(BlockValidationErr::InvalidInput) }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                // No overspending
                if output_value > input_value { return Err(BlockValidationErr::InsufficientInputValue) };

                // Mining fee
                let fee = input_value - output_value;
                total_fee += fee;

                // Add transaction inputs to block_spent
                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            // Coinbase validation
            if coinbase.output_value() < total_fee { return Err(BlockValidationErr::InvalidCoinbaseTransaction) }
            block_created.extend(coinbase.output_hashes());

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        // Add block to blockhain
        println!("Created genesis block -> {:?}", block);
        self.blocks.push(block);

        return Ok(());
    }

    pub fn add_new_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        // Blockchain's last block's index
        let last_block_index = self.blocks.len() - 1;
        // Previous block
        let previous_block = &self.blocks[last_block_index];

        // Check index
        if (last_block_index + 1) as u32 != block.index { return Err(BlockValidationErr::MismatchedIndex) };
        // Check difficulty
        if !block.check_difficulty(&block.hash(), block.difficulty) { return Err(BlockValidationErr::InvalidHash) }
        // Check time
        if block.timestamp <= previous_block.timestamp { return Err(BlockValidationErr::AchronologicalTimestamp)};
        // Check hash
        if block.prev_block_hash != previous_block.hash { return Err(BlockValidationErr::MismatchedPreviousHash)};

        // T R A N S A C T I O N S
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() { return Err(BlockValidationErr::InvalidCoinbaseTransaction) };

            let mut block_spent: HashSet<Vec<u8>> = HashSet::new();
            let mut block_created: HashSet<Vec<u8>> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                
                // No double spending
                if !(&input_hashes - &self.unspent_outputs).is_empty() || !(&input_hashes & &block_spent).is_empty() { return Err(BlockValidationErr::InvalidInput) }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                // No overspending
                if output_value > input_value { return Err(BlockValidationErr::InsufficientInputValue) };

                // Mining fee
                let fee = input_value - output_value;
                total_fee += fee;

                // Add transaction inputs to block_spent
                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            // Coinbase validation
            if coinbase.output_value() < total_fee { return Err(BlockValidationErr::InvalidCoinbaseTransaction) }
            block_created.extend(coinbase.output_hashes());

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        // Add block to blockhain
        println!("Created block -> {:?}", block);
        self.blocks.push(block);

        return Ok(());
    }
}