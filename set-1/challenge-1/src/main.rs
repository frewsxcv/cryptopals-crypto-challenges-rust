extern crate data_encoding;

use data_encoding::{hex, base64};

static INPUT_HEX: &'static [u8] =
    b"49276D206B696C6C696E6720796F757220627261696E206C696B65206120706F69736F6E\
      6F7573206D757368726F6F6D";

static EXPECTED_B64: &'static [u8] =
    b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {
    let input_decoded = hex::decode(INPUT_HEX).unwrap();
    let input_base64 = base64::encode(&input_decoded);
    assert_eq!(EXPECTED_B64, input_base64.as_bytes());
}
