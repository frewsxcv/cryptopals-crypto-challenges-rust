extern crate crypto;
extern crate utils;

use crypto::aes;

static KEY: &'static [u8] = b"YELLOW SUBMARINE";
static INPUT_FILENAME: &'static str = "input.txt";

fn main() {
    let input_file_bytes = utils::read_base64_file(INPUT_FILENAME)
        .expect("could not read base64 file");
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
