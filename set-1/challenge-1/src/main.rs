extern crate base64;
extern crate hex;

use hex::FromHex;

static INPUT_HEX: &'static [u8] =
    b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e\
      6f7573206d757368726f6f6d";

static EXPECTED_B64: &'static [u8] =
    b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {
    let input_hex_str = std::str::from_utf8(INPUT_HEX).unwrap();
    let input_decoded = Vec::from_hex(input_hex_str).unwrap();
    let input_base64 = &base64::u8en(&input_decoded).unwrap();
    assert_eq!(EXPECTED_B64, &input_base64 as &[u8]);
}
