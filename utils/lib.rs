extern crate data_encoding;
extern crate openssl;

use std::ascii::AsciiExt;
use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use data_encoding::{base64, hex};

// https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_the_English_language
fn byte_freq_score(c: u8) -> f32 {
    match c {
        b'a' => 8.167,
        b'b' => 1.492,
        b'c' => 2.782,
        b'd' => 4.253,
        b'e' => 12.70,
        b'f' => 2.228,
        b'g' => 2.015,
        b'h' => 6.094,
        b'i' => 6.966,
        b'j' => 0.153,
        b'k' => 0.772,
        b'l' => 4.025,
        b'm' => 2.406,
        b'n' => 6.749,
        b'o' => 7.507,
        b'p' => 1.929,
        b'q' => 0.095,
        b'r' => 5.987,
        b's' => 6.327,
        b't' => 9.056,
        b'u' => 2.758,
        b'v' => 0.978,
        b'w' => 2.361,
        b'x' => 0.150,
        b'y' => 1.974,
        b'z' => 0.074,
        b @ b'A'...b'Z' => byte_freq_score(b - b'A' + b'a'),
        b' ' => 0.,
        _ => -10.,
    }
}

pub trait BytesExt {
    /// Assumes |self| is ASCII bytes and outputs a String.
    fn ascii_to_string(&self) -> String;

    /// XOR all bytes of |self| with |byte|.
    fn xor_byte(&self, byte: u8, dest: &mut [u8]);

    /// XOR all bytes of |self| with |other|. Panics if lengths differ.
    fn xor_bytes(&self, other: &[u8], dest: &mut [u8]);

    /// XOR all bytes of |self| with |other|. Panics if lengths differ.
    fn xor_bytes_inplace(&mut self, other: &[u8]);

    /// XOR all bytes with the repeating key |key|.
    fn xor_repeating_key(&self, key: &[u8], dest: &mut [u8]);

    /// Calculate the likelihood that |self| is an ASCII English
    /// word/phrase/sentence.
    fn english_score(&self) -> f32;
}

impl BytesExt for [u8] {
    fn ascii_to_string(&self) -> String {
        self.iter().map(|b| *b as char).collect::<String>()
    }

    fn xor_byte(&self, byte: u8, dest: &mut [u8]) {
        for (i, b) in self.iter().enumerate() {
            dest[i] = b ^ byte;
        }
    }

    fn xor_bytes(&self, other: &[u8], dest: &mut [u8]) {
        assert_eq!(self.len(), other.len());
        assert!(dest.len() >= self.len());
        let xor_iter = self.iter().zip(other.iter()).map(|(b1, b2)| b1 ^ b2);
        for (i, xor) in xor_iter.enumerate() {
            dest[i] = xor;
        }
    }

    fn xor_bytes_inplace(&mut self, other: &[u8]) {
        assert_eq!(self.len(), other.len());
        for i in 0..self.len() {
            self[i] ^= other[i];
        }
    }

    fn xor_repeating_key(&self, key: &[u8], dest: &mut [u8]) {
        let xor_iter = self.iter().zip(key.iter().cycle()).map(|(input, key)| {
            input ^ key
        });
        for (i, xor) in xor_iter.enumerate() {
            dest[i] = xor;
        }
    }

    fn english_score(&self) -> f32 {
        self.iter().map(|b| byte_freq_score(*b)).sum()
    }
}

pub fn read_base64_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>, Box<Error>> {
    let mut input_file = File::open(file_path.as_ref())?;
    let mut input_file_bytes = Vec::new();
    input_file.read_to_end(&mut input_file_bytes)?;
    let input_file_bytes = input_file_bytes
        .into_iter()
        .filter(|i| !(*i as char).is_whitespace())
        .collect::<Vec<_>>();
    Ok(base64::decode(&input_file_bytes)?)
}

pub fn read_lines_from_file<P: AsRef<Path>>(file_path: P) -> Vec<String> {
    let mut input_file = File::open(file_path).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();
    input_string
        .lines()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>()
}

pub fn read_hex_lines_from_file<P: AsRef<Path>>(file_path: P) -> Vec<Vec<u8>> {
    let mut input_file = File::open(file_path).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();
    input_string
        .lines()
        .map(|str_| str_.as_bytes())
        .map(|bytes| bytes.to_ascii_uppercase())
        .map(|bytes| {
            hex::decode(&bytes).expect("encountered invalid hex")
        })
        .collect::<Vec<_>>()
}

pub mod aes_128_ecb {
    use openssl::symm;

    pub fn encrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
        common(ciphertext, key, symm::Mode::Encrypt)
    }

    pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
        common(ciphertext, key, symm::Mode::Decrypt)
    }

    fn common(in_: &[u8], key: &[u8], mode: symm::Mode) -> Vec<u8> {
        let cipher = symm::Cipher::aes_128_ecb();
        let mut crypter = symm::Crypter::new(cipher, mode, key, None).unwrap();
        crypter.pad(false);

        let mut out = vec![0; in_.len() + cipher.block_size()];
        let count1 = crypter
            .update(&in_, &mut out)
            .unwrap();
        let count2 = crypter.finalize(&mut out).unwrap();
        out.truncate(count1 + count2);
        out
    }
}
