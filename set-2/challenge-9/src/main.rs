fn main() {
    assert_eq!(
        pad(b"YELLOW SUBMARINE".to_vec(), 20),
        b"YELLOW SUBMARINE\x04\x04\x04\x04"
    );

    assert_eq!(
        pad(b"YELLOW".to_vec(), 6),
        b"YELLOW\x06\x06\x06\x06\x06\x06"
    );
}

fn pad(mut plaintext: Vec<u8>, block_len: usize) -> Vec<u8> {
    let pad_len = block_len - (plaintext.len() % block_len);
    let new_len = plaintext.len() + pad_len;
    assert!(pad_len < 256);
    plaintext.resize(new_len, pad_len as u8);
    plaintext
}
