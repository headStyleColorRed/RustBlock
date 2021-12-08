mod block;

fn main() {
    let block = block::Block::new(0, 0, vec![0, 32], 0, String::from("Genesis Block"));
    
    println!("{:?}", &block);
}
