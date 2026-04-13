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
}
