extern crate openssl;
extern crate utils;

use openssl::symm;
use std::io::Read;
use std::fs;

static KEY: &[u8] = b"YELLOW SUBMARINE";
static INPUT_FILENAME: &str = "input.txt";
static SOLUTION_FILENAME: &str = "solution.txt";

fn main() {
    let input_file_bytes =
        utils::read_base64_file(INPUT_FILENAME).expect("could not read base64 file");
    let cipher = symm::Cipher::aes_128_cbc();
    let mut decryptor = symm::Crypter::new(
        cipher,
        symm::Mode::Decrypt,
        KEY,
        Some(&vec![0; cipher.iv_len().unwrap()]),
    ).unwrap();
    decryptor.pad(false);

    let mut output_bytes = vec![0; input_file_bytes.len() + cipher.block_size()];
    let count1 = decryptor
        .update(&input_file_bytes, &mut output_bytes)
        .unwrap();
    let count2 = decryptor.finalize(&mut output_bytes).unwrap();
    output_bytes.truncate(count1 + count2);
    assert_eq!(output_bytes, solution());
}

fn solution() -> Vec<u8> {
    let mut bytes = vec![];
    let mut file = fs::File::open(SOLUTION_FILENAME).unwrap();
    file.read_to_end(&mut bytes).unwrap();
    bytes
}
