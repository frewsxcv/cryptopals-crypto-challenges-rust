extern crate data_encoding;
#[macro_use]
extern crate lazy_static;
extern crate utils;

use data_encoding::BASE64;
use std::collections::HashMap;
use utils::aes_128_ecb;

const KEY: [u8; 16] = *b"58ss8aospoaisjef";

lazy_static! {
    static ref PLAINTEXT_SUFFIX: Vec<u8> = BASE64
        .decode(
            b"Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
              aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
              dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
              YnkK"
        )
        .unwrap();
}

fn main() {
    part_1();
    part_2();
    part_3();
    part_4();
    part_5();
}

// when input goes from length 5 bytes to length 16 bytes, the output increases by 16 bytes
fn part_1() {
    assert_eq!(
        encryption_oracle(b"AAAAAA").len() - encryption_oracle(b"AAAAA").len(),
        aes_128_ecb::BLOCK_SIZE,
    );
}

// when fed a 2 × block length (16 bytes) input, the first two block lengths will be identical
fn part_2() {
    let ciphertext = encryption_oracle(b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    assert_eq!(
        ciphertext[..aes_128_ecb::BLOCK_SIZE],
        ciphertext[aes_128_ecb::BLOCK_SIZE..][..aes_128_ecb::BLOCK_SIZE]
    );
}

// if we feed the oracle fifteen 'A's, the first byte of the unknown string gets included as the
// last byte of the first block. we can generate all 256 ciphertexts and match to deterrmine
// the first character of the unknown string.
fn part_3() { }

fn part_4() {
    let prefix = [b'A'; 15];

    let ciphertext = encryption_oracle(&prefix[..]);
    let ciphertext_first_block = &ciphertext[..aes_128_ecb::BLOCK_SIZE];

    let lookup = build_lookup(&prefix);

    assert_eq!(b'R', lookup[ciphertext_first_block]);
}

fn part_5() {
    let mut prefix = vec![b'A'; 14];

    let ciphertext = encryption_oracle(&prefix);
    let ciphertext_first_block = &ciphertext[..aes_128_ecb::BLOCK_SIZE];

    prefix.push(b'R');
    let lookup = build_lookup(&prefix);

    assert_eq!(b'o', lookup[ciphertext_first_block]);
}

fn build_lookup(prefix: &[u8]) -> HashMap<Vec<u8>, u8> {
    assert_eq!(prefix.len(), aes_128_ecb::BLOCK_SIZE - 1);
    let mut map = HashMap::new();
    let mut input = prefix.to_vec();
    input.push(0);
    for c in 0..=255 {
        input[15] = c;
        let mut ciphertext = encryption_oracle(&input);
        ciphertext.truncate(aes_128_ecb::BLOCK_SIZE);
        map.insert(ciphertext, c);
    }
    map
}

fn encryption_oracle(my_string: &[u8]) -> Vec<u8> {
    let mut concat = Vec::with_capacity(my_string.len() + PLAINTEXT_SUFFIX.len());
    concat.extend_from_slice(my_string);
    concat.extend_from_slice(&PLAINTEXT_SUFFIX);
    let concat = utils::pkcs7::pad(concat, aes_128_ecb::BLOCK_SIZE);
    aes_128_ecb::encrypt(&concat, &KEY)
}
