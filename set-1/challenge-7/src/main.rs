extern crate utils;

use std::io::Read;
use std::fs;

static KEY: &[u8] = b"YELLOW SUBMARINE";
static INPUT_FILENAME: &str = "input.txt";
static SOLUTION_FILENAME: &str = "solution.txt";

fn main() {
    let ciphertext = utils::read_base64_file(INPUT_FILENAME).expect("could not read base64 file");
    assert_eq!(utils::aes_128_ecb::decrypt(&ciphertext, KEY), solution());
}

fn solution() -> Vec<u8> {
    let mut bytes = vec![];
    let mut file = fs::File::open(SOLUTION_FILENAME).unwrap();
    file.read_to_end(&mut bytes).unwrap();
    bytes
}
