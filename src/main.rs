use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;

mod utils;

fn main() {
    let mut args = env::args().skip(1);

    // Extract the file path from the arguments
    let file_path = args.next().unwrap_or_else(|| {
        eprintln!("Error: Missing file path argument.");
        std::process::exit(1);
    });

    // Attempt to read the input file
    let contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error: Error reading file {}: {}", file_path, err);
            std::process::exit(1);
        }
    };

    // Create a hashmap mapping each character with its frequency (i.e. how many times it appears in the text)
    let freq_map = utils::analyze_frequency(&contents);

    // Build the Huffman tree using the frequency map
    let tree = utils::HuffmanNode::build_tree(&freq_map);

    // Create the prefix-code table and encode the tree
    let mut codes = HashMap::new();
    tree.encode(String::new(), &mut codes);

    let encoded_text = utils::encode_text(&contents, &codes);

    // Create a new file adding 'encoded' to the original filename
    let filename: Vec<_> = file_path.split(".").collect();
    let path = String::from(filename[0].to_owned() + "_encoded.txt");

    let mut file = match fs::File::create(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error: Error creating file: {}", err);
            std::process::exit(1);
        }
    };

    // Write the encoded text to the newly created file
    match file.write_all(&encoded_text) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error: Error writing to file: {}", err);
            std::process::exit(1);
        }
    };
}
