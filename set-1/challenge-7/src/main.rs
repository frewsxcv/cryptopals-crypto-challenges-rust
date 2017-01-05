extern crate crypto;
extern crate data_encoding;

use crypto::aes;
use crypto::buffer::{ReadBuffer, WriteBuffer};
use data_encoding::base64;

use std::io::Read;
use std::fs::File;

static KEY: &'static [u8] = b"YELLOW SUBMARINE";
static INPUT_FILENAME: &'static str = "input.txt";

fn main() {
    let mut input_file = File::open(INPUT_FILENAME).unwrap();
    let mut input_file_bytes = Vec::new();
    input_file.read_to_end(&mut input_file_bytes).unwrap();
    let input_file_bytes = input_file_bytes.into_iter()
        .filter(|i| !(*i as char).is_whitespace())
        .collect::<Vec<_>>();
    let input_file_bytes = base64::decode(&input_file_bytes)
        .expect("could not base64 decode input file");
    let mut decryptor =
        aes::ecb_decryptor(aes::KeySize::KeySize128, KEY, crypto::blockmodes::NoPadding);

    let mut input_buffer = crypto::buffer::RefReadBuffer::new(&input_file_bytes);
    let mut output_bytes = vec![9; input_file_bytes.len()];

    {
        let mut output_buffer = crypto::buffer::RefWriteBuffer::new(&mut output_bytes);
        let result = decryptor.decrypt(&mut input_buffer, &mut output_buffer, true)
            .expect("AES decrypt failed");
        match result {
            crypto::buffer::BufferResult::BufferUnderflow => (),
            crypto::buffer::BufferResult::BufferOverflow => panic!("output buffer overflow"),
        }
    }

    let s = String::from_utf8(output_bytes).expect("invalid utf8 string");

    println!("{}", s);
}
