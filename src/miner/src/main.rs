use lib::types::Block;
use lib::util::Saveable;
use std::env;
use std::process::exit;

fn main() {
    // parse the block path and steps count from the first and second argument
    let (path, steps) = if let (Some(arg), Some(arg1)) = (env::args().nth(1), env::args().nth(2)) {
        (arg, arg1)
    } else {
        eprintln!("Usage: miner <block_file> <steps>");
        exit(1);
    };
    // parse steps count
    let steps: usize = if let Ok(s @ 1..=usize::MAX) = steps.parse() {
        s
    } else {
        eprintln!("<steps> should be a positive integer");
        exit(1);
    };
    let og_block = Block::load_from_file(path).expect("Failed to load block");
    let mut block = og_block.clone();

    while !block.header.mine(steps) {
        println!("mining...");
    }
    println!("original:{:#?}", og_block);
    println!("hash: {}", og_block.header.hash());
    println!("final: {:#?}", block);
    println!("hash: {}", block.header.hash());
}
