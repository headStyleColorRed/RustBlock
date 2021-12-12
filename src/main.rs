use library::*;

fn main() {
    let difficulty: Difficulty = Difficulty::Medium;

    // Create genesis block
    let mut genesis_block = create_genesis_block(&difficulty);

    // Mine block
    genesis_block.mine();

    // Create blockchain
    let mut blockchain = Blockchain::new();

    // Add Genesis block
    blockchain.add_genesis_block(genesis_block).unwrap_or_else(|err| panic!("{:?}", err));

    // Last block hash
    let genesis_block: &Block = blockchain.blocks.first().expect("Error getting genesis block hash");
    let mut previous_hash: Vec<u8> = genesis_block.hash.clone();

    // Start blockchain loop
    let mut index = 1;
    loop {
        // Testing
        let mut transactions: Vec<Transaction> = vec![];
        if index == 1 {
            transactions.push(Transaction::new("AliceAddress", "BobAddress" , 30, blockchain.blocks[index - 1].transactions[0].outputs.clone()));
        }
        // End testing

        // Create block
        let mut block = Block::new(index as u32, now(), previous_hash, 26661, transactions, difficulty.hash());

        // Mine block
        block.mine();

        // Save block's hash
        previous_hash = block.hash();

        match blockchain.add_new_block(block) {
            Ok(_) => "",
            Err(err) => panic!("{:?}", err),
        };

        index += 1
    }
}

fn create_genesis_block(difficulty: &Difficulty) -> Block {
    let index: u32 = 0;
    let timestamp: u128 = now();
    let hash: Vec<u8> = vec![0, 32];
    let nonce: u64 = 0;
    let transactions: Vec<Transaction> = vec![Transaction {
        inputs: vec![],
        outputs: vec![
            Output {
                to_addr: "AliceAddress".to_owned(),
                from_addr: "coinbase".to_owned(),
                value: 50,
            },
            Output {
                to_addr: "BobAddress".to_owned(),
                from_addr: "coinbase".to_owned(),
                value: 7,
            },
        ],
    }];
    let difficulty: u128 = difficulty.hash();

    // Create block
    Block::new(index, timestamp, hash, nonce, transactions, difficulty)
}
