use library::*;

fn main() {
    // Genesis block variables
    let index: u32 = 0;
    let timestamp: u128 = now();
    let hash: Vec<u8> = vec![0, 32];
    let nonce: u64 = 0;
    let payload: String = String::from("Genesis Block");
    let difficulty: u128 = 0x00000ffffffffffffffffffffffffffff;

    // Create block
    let mut block = Block::new(index, timestamp, hash, nonce, payload, difficulty);

    // Mine block
    block.mine();

    println!("Created genesis block {:?}", block);

    // Create blockchain
    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    // Last block hash
    let genesis_block: &Block = blockchain.blocks.first().expect("Error getting genesis block hash");
    let mut previous_hash: Vec<u8> = genesis_block.hash.clone();

    // Start blockchain loop
    for index in 1..=10 {
        // Previous hash

        // Create block
        let mut block = Block::new(index, now(), previous_hash, 26661, String::from(format!("Block nº: {}", index)), difficulty);

        // Mine block
        block.mine();

        // Save block's hash
        previous_hash = block.hash();

        println!("Created block {:?}", block);

        // Add block to blockchain
        blockchain.blocks.push(block);
    }
}

