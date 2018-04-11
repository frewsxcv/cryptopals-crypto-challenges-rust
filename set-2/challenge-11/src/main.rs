extern crate rand;
extern crate utils;

use rand::Rng;
use std::iter;

type Key = [u8; 16];
type Iv = [u8; 16];

const BLOCK_SIZE: usize = 16;

#[derive(PartialEq, Eq, Debug)]
enum Mode {
    Ecb,
    Cbc,
}

fn main() {
    let plaintext = iter::repeat(b'a').take(BLOCK_SIZE * 4).collect::<Vec<_>>();
    let (ciphertext, mode) = encryption_oracle(&plaintext[..]);
    let second_block = &ciphertext[BLOCK_SIZE..(BLOCK_SIZE * 2)];
    let third_block = &ciphertext[(BLOCK_SIZE * 2)..(BLOCK_SIZE * 3)];
    if second_block == third_block {
        assert_eq!(mode, Mode::Ecb);
    } else {
        assert_eq!(mode, Mode::Cbc);
    }
}

// Returning the Mode so we can verify the solution
fn encryption_oracle(plaintext: &[u8]) -> (Vec<u8>, Mode) {
    let mut rng = rand::os::OsRng::new().unwrap();
    let prefix_len = rng.gen_range(5, 11);
    let suffix_len = rng.gen_range(5, 11);
    let padded = vec![0; prefix_len + plaintext.len() + suffix_len];
    let mut padded = utils::pkcs7::pad(padded, BLOCK_SIZE);
    rng.fill_bytes(&mut padded[..prefix_len]);
    padded[prefix_len..][..plaintext.len()].copy_from_slice(&plaintext);
    rng.fill_bytes(&mut padded[prefix_len + plaintext.len()..]);
    if rng.gen_weighted_bool(2) {
        (
            utils::aes_128_ecb::encrypt(&padded, &rand::random::<Key>()),
            Mode::Ecb,
        )
    } else {
        (
            utils::aes_128_cbc::encrypt(&padded, &rand::random::<Key>(), &rand::random::<Iv>()),
            Mode::Cbc,
        )
    }
}
