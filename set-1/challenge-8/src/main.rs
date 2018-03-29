extern crate utils;

use utils::{read_hex_lines_from_file, read_lines_from_file};
use std::collections::HashMap;

static INPUT_FILENAME: &str = "input.txt";
const BLOCK_LEN: usize = 16;

fn main() {
    let lines = read_hex_lines_from_file(INPUT_FILENAME);
    let ecb_index = lines
        .into_iter()
        .map(|line_bytes| {
            line_bytes
                .chunks(BLOCK_LEN)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .map(|chunks| {
            let mut hash_map = HashMap::new();
            for chunk in chunks {
                *hash_map.entry(chunk).or_insert(0) += 1;
            }
            hash_map
        })
        .map(|map| map.values().map(ToOwned::to_owned).max().unwrap())
        .enumerate()
        .max_by_key(|&(_, n)| n)
        .unwrap()
        .0;
    println!(
        "row: {}\ntext: {}",
        ecb_index,
        read_lines_from_file(INPUT_FILENAME)[ecb_index]
    );
}
