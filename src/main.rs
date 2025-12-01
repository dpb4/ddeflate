use bitvec::prelude::*;
use std::collections::HashMap;
mod huffman;

fn main() {
    println!("Hello, world!");

    let initial_map = HashMap::from([
        ("A".to_string(), bitvec![0, 1, 0]),
        ("B".to_string(), bitvec![0, 1, 1]),
        ("C".to_string(), bitvec![1, 0, 0]),
        ("D".to_string(), bitvec![1, 0, 1]),
        ("E".to_string(), bitvec![1, 1, 0]),
        ("F".to_string(), bitvec![0, 0]),
        ("G".to_string(), bitvec![1, 1, 1, 0]),
        ("H".to_string(), bitvec![1, 1, 1, 1]),
    ]);

    dbg!(huffman::canonicalize(&initial_map));
}
