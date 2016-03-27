extern crate data_encoding;

use data_encoding::hex;

const INPUT1_HEX: &'static [u8] = b"1C0111001F010100061A024B53535009181C";
const INPUT2_HEX: &'static [u8] = b"686974207468652062756C6C277320657965";
const EXPECTED_HEX: &'static [u8] = b"746865206B696420646F6E277420706C6179";

fn xor_bytestrings(dest: &mut [u8], src1: &[u8], src2: &[u8]) {
    assert_eq!(src1.len(), src2.len());
    assert!(dest.len() >= src1.len());
    let xor_iter = src1.iter()
                       .zip(src2.iter())
                       .map(|(b1, b2)| b1 ^ b2);
    for (i, xor) in xor_iter.enumerate() {
        dest[i] = xor;
    }
}

fn main() {
    let input1 = hex::decode(INPUT1_HEX).unwrap();
    let input2 = hex::decode(INPUT2_HEX).unwrap();
    let mut result = vec![0; input1.len()];
    xor_bytestrings(&mut result, &input1, &input2);
    let result_hex = hex::encode(&result);
    assert_eq!(result_hex.as_bytes(), EXPECTED_HEX);
}
