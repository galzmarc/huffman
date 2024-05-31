use huffman::*;

use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let mut args = env::args().skip(1);

    // Extract the file path from the arguments
    let file_path = args.next().unwrap_or_else(|| {
        eprintln!("Error: Missing file path argument.");
        std::process::exit(1);
    });

    // Extract mode (encode or decode) from the arguments
    let mode = args.next().unwrap_or_else(|| {
        eprintln!("Error: Missing mode argument.");
        std::process::exit(1);
    });

    match mode.as_str() {
        "--e" => encoder(&file_path),
        "--d" => decoder(&file_path),
        _ => eprintln!("Error: Invalid mode.")
    }
}

fn encoder(file_path: &str) {
    // Attempt to read the input file
    let contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error: Error reading file {}: {}", file_path, err);
            std::process::exit(1);
        }
    };

    // Encode text
    let encoded_text = encode(contents);

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

fn decoder(file_path: &str) {
    // Attempt to read the input file
    let contents = match fs::read(&file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error: Error reading file {}: {}", file_path, err);
            std::process::exit(1);
        }
    };

    // Decode text
    let decoded_text = decode(&contents);

    println!("{}", decoded_text);
}