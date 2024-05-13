use std::env;
use std::fs;

mod utils;

fn main() {
    let mut args = env::args().skip(1);

    // Extract the file path from the arguments
    let file_path = args.next().unwrap_or_else(|| {
        eprintln!("Error: Missing file path argument.");
        std::process::exit(1);
    });

    // Attempt to read the file
    let contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error: Error reading file {}: {}", file_path, err);
            std::process::exit(1);
        }
    };

    // Create a hashmap mapping each character with its frequency (i.e. how many times it appears in the text)
    let freq_map = utils::analyze_frequency(&contents);

    let tree = utils::HuffmanNode::new(&freq_map);
    println!("{:?}", tree);
}