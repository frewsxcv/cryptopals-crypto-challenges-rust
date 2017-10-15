extern crate openssl;
extern crate utils;

use openssl::symm;
use std::io::Read;
use std::fs;
use utils::BytesExt;

static KEY: &[u8] = b"YELLOW SUBMARINE";
static INPUT_FILENAME: &str = "input.txt";
static SOLUTION_FILENAME: &str = "solution.txt";

fn main() {
    let ciphertext =
        utils::read_base64_file(INPUT_FILENAME).expect("could not read base64 file");
    let cipher = symm::Cipher::aes_128_ecb();

    let mut decryptor = symm::Crypter::new(
        cipher,
        symm::Mode::Decrypt,
        KEY,
        None,
    ).unwrap();
    decryptor.pad(false);

    let mut plaintext: Vec<u8> = vec![];
    let mut prev_ciphertext_block = &vec![0; cipher.block_size()][..]; // Initial value is the IV
    let mut decrypted_buf = vec![0; 2 * cipher.block_size()];
    for ciphertext_block in ciphertext.chunks(cipher.block_size()) {
        let count1 = decryptor
            .update(&ciphertext_block, &mut decrypted_buf)
            .unwrap();
        let count2 = decryptor.finalize(&mut decrypted_buf).unwrap();
        decrypted_buf[..count1 + count2].xor_bytes_inplace(&prev_ciphertext_block);
        prev_ciphertext_block = ciphertext_block;
        plaintext.extend(&decrypted_buf[..count1 + count2]);
    }

    assert_eq!(plaintext, solution());
}

fn solution() -> Vec<u8> {
    let mut bytes = vec![];
    let mut file = fs::File::open(SOLUTION_FILENAME).unwrap();
    file.read_to_end(&mut bytes).unwrap();
    bytes
}
