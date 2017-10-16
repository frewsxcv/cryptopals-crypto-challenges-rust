extern crate utils;

use std::io::Read;
use std::fs;

static KEY: &[u8] = b"YELLOW SUBMARINE";
static INPUT_FILENAME: &str = "input.txt";
static SOLUTION_FILENAME: &str = "solution.txt";

fn main() {
    let ciphertext = utils::read_base64_file(INPUT_FILENAME).expect("could not read base64 file");
    let iv = [0; utils::aes_128_cbc::BLOCK_SIZE];

    // Ensure we can decrypt the supplied input file
    let cleartext = utils::aes_128_cbc::decrypt(&ciphertext, KEY, &iv[..]);
    assert_eq!(cleartext, solution());

    // Ensure our encryption algorithm works by encrypting and confirming it matches the original
    let ciphertext2 = utils::aes_128_cbc::encrypt(&cleartext, KEY, &iv);
    assert_eq!(ciphertext, ciphertext2);
}

fn solution() -> Vec<u8> {
    let mut bytes = vec![];
    let mut file = fs::File::open(SOLUTION_FILENAME).unwrap();
    file.read_to_end(&mut bytes).unwrap();
    bytes
}
