use std::collections::HashMap;

use encoder::{encode_text, generate_codes};
use decoder::decode_text;

mod encoder;
mod decoder;

fn analyze_frequency(contents: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();

    for character in contents.chars() {
        let count = freq_map.entry(character).or_insert(0);
        *count += 1;
    }

    freq_map
}

pub fn encode(contents: String) -> Vec<u8> {
    // Create a hashmap mapping each character with its frequency (i.e. how many times it appears in the text)
    let freq_map = analyze_frequency(&contents);
    // Create a hashmap mapping each character with its canonical Huffman code
    let (codes, length_char_tuples) = generate_codes(&freq_map);
    // Encode text
    let encoded_text = encode_text(&contents, &codes, &length_char_tuples);
    encoded_text
}

pub fn decode(encoded_data: &[u8]) -> String {
    decode_text(encoded_data)
}