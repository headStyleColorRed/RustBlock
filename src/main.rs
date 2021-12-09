use library::*;

fn main() {
    let mut block = Block::new(0, 0, vec![0, 32], 0, String::from("Genesis Block"));
    block.hash = block.hash();

    println!("{:?}", block);
}

