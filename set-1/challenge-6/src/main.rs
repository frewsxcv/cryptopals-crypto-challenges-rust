extern crate hamming;
extern crate utils;

use utils::BytesExt;


static INPUT_FILENAME: &'static str = "input.txt";

fn likely_keysizes(input: &[u8]) -> Vec<usize> {
    #[derive(Debug)]
    struct HammingResult {
        distance: f32,
        keysize: usize,
    };
    let keysizes = 2..60;
    let mut results = Vec::with_capacity(input.len());
    for keysize in keysizes {
        let a = &input[..keysize];
        let b = &input[keysize..keysize * 2];
        let distance = hamming::distance(a, b) as f32 / keysize as f32;
        results.push(HammingResult {
            distance: distance,
            keysize: keysize,
        });
    }
    results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    results.into_iter()
           .map(|result| result.keysize)
           .collect()
}

/// Returns key and normalized score
fn xor_key_with_best_english_score(input: &[u8]) -> (u8, f32) {
    let mut best = (0f32, 0u8, String::new());
    let mut input_xor = vec![0; input.len()];
    for i in 0..255 {
        input.xor_byte(i, &mut input_xor);
        let score = input_xor.english_score();
        if score > best.0 {
            let s = input_xor.ascii_to_string();
            best = (score, i, s);
        }
    }
    (best.1, best.0 / input.len() as f32)
}

fn find_best_key_at_keysize(input: &[u8], keysize: usize) -> (Vec<u8>, f32) {
    let mut transposed = vec![Vec::<u8>::new(); keysize];

    for chunk in input.chunks(keysize) {
        for (i, byte) in chunk.iter().enumerate() {
            transposed[i].push(byte.clone());
        }
    }

    let mut key = vec![0; keysize];
    let mut score = 0f32;
    for (i, t) in transposed.into_iter().enumerate() {
        let (key_part, score_part) = xor_key_with_best_english_score(&t);
        key[i] = key_part;
        score += score_part;
    }

    (key, score / keysize as f32)
}

fn main() {
    let input = utils::read_base64_file(INPUT_FILENAME)
        .expect("could not read base64 file");

    let keysizes = likely_keysizes(&input);

    let mut v = vec![];
    for keysize in &keysizes[..7] {
        v.push(find_best_key_at_keysize(&input, keysize.clone()));
    }
    v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let ref xor_key = v[0].0;
    let mut result = vec![0; input.len()];
    input.xor_repeating_key(xor_key, &mut result);

    println!("{}", result.ascii_to_string());
}
