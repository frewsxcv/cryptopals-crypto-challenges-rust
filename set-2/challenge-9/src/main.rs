fn main() {
    assert_eq!(
        pad(b"YELLOW SUBMARINE".to_vec(), 20, b'\x04'),
        b"YELLOW SUBMARINE\x04\x04\x04\x04"
    )
}

fn pad(mut plaintext: Vec<u8>, block_len: usize, pad_val: u8) -> Vec<u8> {
    let pad_len = block_len - (plaintext.len() % block_len);
    let new_len = plaintext.len() + pad_len;
    plaintext.resize(new_len, pad_val);
    plaintext
}
