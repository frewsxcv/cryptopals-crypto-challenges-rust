extern crate data_encoding;
extern crate utils;

use data_encoding::hex;
use std::ascii::AsciiExt;
use std::fs::File;
use std::io::Read;
use utils::BytesExt;


static INPUT_FILENAME: &'static str = "input.txt";

fn main() {
    let mut input_file = File::open(INPUT_FILENAME).unwrap();
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();
    let mut best = (0f32, 0u8, String::new());
    for line in input_string.lines() {
        let upper_line = line.as_bytes().to_ascii_uppercase();
        let line_bytes = hex::decode(&upper_line).unwrap();
        let mut input_xor = vec![0; line_bytes.len()];
        for i in 0..255 {
            line_bytes.xor_byte(i, &mut input_xor);
            let score = input_xor.english_score();
            if score > best.0 {
                let s = input_xor.ascii_to_string();
                best = (score, i, s);
            }
        }
    }
    println!("score: '{}'", best.0);
    println!("key: '{}'", best.1 as char);
    println!("plaintext: {}", best.2);
}
