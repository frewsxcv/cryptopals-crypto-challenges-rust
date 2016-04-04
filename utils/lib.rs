#![feature(iter_arith)]

// https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_the_English_language
pub fn byte_freq_score(c: u8) -> f32 {
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

pub fn bytestring_xor(dest: &mut [u8], src: &[u8], xor: u8) {
    for (i, b) in src.iter().enumerate() {
        dest[i] = b ^ xor;
    }
}

pub fn bytestring_score(bytes: &[u8]) -> f32 {
    bytes.iter()
         .map(|b| byte_freq_score(*b))
         .sum()
}

pub fn bytestring_to_string(bytes: &[u8]) -> String {
    bytes.iter()
         .map(|b| *b as char)
         .collect::<String>()
}

pub fn xor_bytestrings(dest: &mut [u8], src1: &[u8], src2: &[u8]) {
    assert_eq!(src1.len(), src2.len());
    assert!(dest.len() >= src1.len());
    let xor_iter = src1.iter()
                       .zip(src2.iter())
                       .map(|(b1, b2)| b1 ^ b2);
    for (i, xor) in xor_iter.enumerate() {
        dest[i] = xor;
    }
}
