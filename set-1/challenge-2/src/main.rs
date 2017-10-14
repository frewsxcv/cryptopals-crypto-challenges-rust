extern crate data_encoding;
extern crate utils;

use data_encoding::hex;
use utils::BytesExt;

const INPUT1_HEX: &[u8] = b"1C0111001F010100061A024B53535009181C";
const INPUT2_HEX: &[u8] = b"686974207468652062756C6C277320657965";
const EXPECTED_HEX: &[u8] = b"746865206B696420646F6E277420706C6179";

fn main() {
    let input1 = hex::decode(INPUT1_HEX).unwrap();
    let input2 = hex::decode(INPUT2_HEX).unwrap();
    let mut result = vec![0; input1.len()];
    input1.xor_bytes(&input2, &mut result);
    let result_hex = hex::encode(&result);
    assert_eq!(result_hex.as_bytes(), EXPECTED_HEX);
}
