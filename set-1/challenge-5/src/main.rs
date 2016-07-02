extern crate data_encoding;
extern crate utils;

use data_encoding::hex;
use utils::BytesExt;

const INPUT: &'static [u8] =
    b"Burning 'em, if you ain't quick and nimble\n\
      I go crazy when I hear a cymbal";
const XOR_KEY: [u8; 3] = [b'I', b'C', b'E'];
const EXPECTED_HEX: &'static [u8] =
    b"0B3637272A2B2E63622C2E69692A23693A2A3C6324202D623D63343C2A26226324272765272\
      A282B2F20430A652E2C652A3124333A653E2B2027630C692B20283165286326302E27282F";

fn main() {
    let mut result_xor = vec![0; INPUT.len()];
    INPUT.xor_repeating_key(&XOR_KEY, &mut result_xor);
    let result_hex = hex::encode(&result_xor);
    assert_eq!(result_hex.as_bytes(), EXPECTED_HEX);
}
