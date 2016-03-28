#![feature(iter_arith)]

extern crate data_encoding;

use data_encoding::hex;

static INPUT_HEX: &'static [u8] =
    b"1B37373331363F78151B7F2B783431333D78397828372D363C78373E783A393B3736";

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
        b' ' => 0.,
        _ => -10.,
    }
}

fn bytestring_xor(dest: &mut [u8], src: &[u8], xor: u8) {
    for (i, b) in src.iter().enumerate() {
        dest[i] = b ^ xor;
    }
}

fn bytestring_score(bytes: &[u8]) -> f32 {
    bytes.iter()
         .map(|b| byte_freq_score(*b))
         .sum()
}

fn bytestring_to_string(bytes: &[u8]) -> String {
    bytes.iter()
         .map(|b| *b as char)
         .collect::<String>()
}

fn main() {
    let input = hex::decode(INPUT_HEX).unwrap();
    let mut input_xor = vec![0; input.len()];
    // (top score, key, shifted bytes as string)
    let mut best = (0f32, 0u8, String::new());
    for i in 0..255 {
        bytestring_xor(&mut input_xor, &input, i);
        let score = bytestring_score(&input_xor);
        if score > best.0 {
            let s = bytestring_to_string(&input_xor);
            best = (score, i, s);
        }
    }
    println!("key: '{}'", best.1 as char);
    println!("plaintext: {}", best.2);
}
