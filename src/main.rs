use std::collections::HashMap;

mod huffman;

fn main() {
    println!("Hello, world!");

    let initial_map = HashMap::from([
        ("A".to_string(), "010".to_string()),
        ("B".to_string(), "011".to_string()),
        ("C".to_string(), "100".to_string()),
        ("D".to_string(), "101".to_string()),
        ("E".to_string(), "110".to_string()),
        ("F".to_string(), "00".to_string()),
        ("G".to_string(), "1110".to_string()),
        ("H".to_string(), "1111".to_string()),
    ]);

    dbg!(huffman::canonicalize(&initial_map));
}
