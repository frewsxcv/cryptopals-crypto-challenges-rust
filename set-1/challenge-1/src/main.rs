extern crate data_encoding;

use data_encoding::{HEXUPPER, BASE64};

static INPUT_HEX: &[u8] = b"49276D206B696C6C696E6720796F757220627261696E206C696\
    B65206120706F69736F6E6F7573206D757368726F6F6D";

static EXPECTED_B64: &[u8] = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29u\
    b3VzIG11c2hyb29t";

fn main() {
    let input_decoded = HEXUPPER.decode(INPUT_HEX).unwrap();
    let input_base64 = BASE64.encode(&input_decoded);
    assert_eq!(EXPECTED_B64, input_base64.as_bytes());
}
