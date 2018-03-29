extern crate utils;

use utils::{read_hex_lines_from_file, BytesExt};

static INPUT_FILENAME: &str = "input.txt";

fn main() {
    let mut best = (0f32, 0u8, String::new());
    for line_bytes in read_hex_lines_from_file(INPUT_FILENAME) {
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
