extern crate utils;

fn main() {
    assert_eq!(
        utils::pkcs7::pad(b"YELLOW SUBMARINE".to_vec(), 20),
        b"YELLOW SUBMARINE\x04\x04\x04\x04"
    );
    assert_eq!(
        utils::pkcs7::unpad(b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec()),
        b"YELLOW SUBMARINE"
    );

    assert_eq!(
        utils::pkcs7::pad(b"YELLOW".to_vec(), 6),
        b"YELLOW\x06\x06\x06\x06\x06\x06"
    );
    assert_eq!(
        utils::pkcs7::unpad(b"YELLOW\x06\x06\x06\x06\x06\x06".to_vec()),
        b"YELLOW"
    );
}
