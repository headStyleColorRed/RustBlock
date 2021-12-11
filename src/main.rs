use library::*;

fn main() {
    // Create block
    let mut block = Block::new(0, 0, vec![0, 32], 26661, String::from("Genesis Block"), 0x00000ffffffffffffffffffffffffffff);

    // Save hash
    block.hash = block.hash();

    // Mine block
    block.mine();

    println!("{:?}", block);
}

