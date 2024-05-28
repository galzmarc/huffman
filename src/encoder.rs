use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
struct Node {
    char: Option<char>,
    freq: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the ordering for min-heap behavior
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_tree(
    root: Option<&Box<Node>>,
    code_length: String,
    code_map: &mut HashMap<usize, Vec<char>>,
) {
    if let Some(node) = root {
        if node.left.is_none() && node.right.is_none() {
            code_map
                .entry(code_length.len())
                .or_insert_with(Vec::new)
                .push(node.char.unwrap());
        } else {
            build_tree(node.left.as_ref(), code_length.clone() + "0", code_map);
            build_tree(node.right.as_ref(), code_length + "1", code_map);
        }
    }
}

pub fn generate_codes(freq_map: &HashMap<char, usize>) -> (HashMap<char, String>, Vec<(usize, Vec<char>)>) {
    // Build a priority queue
    let mut priority_queue: BinaryHeap<Node> = BinaryHeap::new();
    for (&char, &freq) in freq_map {
        priority_queue.push(Node {
            char: Some(char),
            freq,
            left: None,
            right: None,
        });
    }

    // Iterate through the priority queue to build the Huffman Tree
    while priority_queue.len() > 1 {
        let left = priority_queue.pop().unwrap();
        let right = priority_queue.pop().unwrap();

        let merged = Node {
            char: None,
            freq: left.freq + right.freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        priority_queue.push(merged);
    }

    let root = priority_queue.pop().unwrap();
    let mut code_map: HashMap<usize, Vec<char>> = HashMap::new();
    build_tree(Some(&Box::new(root)), String::new(), &mut code_map);

    // Generate Canonical Huffman codes
    let mut canonical_map: HashMap<char, String> = HashMap::new();
    let mut c_code: usize = 0;
    let mut lengths: Vec<_> = code_map.keys().cloned().collect();
    lengths.sort();

    let mut length_char_tuples = Vec::new();

    for length in lengths {
        let mut chars = code_map[&length].clone();
        chars.sort();
        length_char_tuples.push((length, chars.clone()));
        for char in chars {
            canonical_map.insert(char, format!("{:0length$b}", c_code, length = length));
            c_code += 1;
        }
        c_code <<= 1;
    }

    (canonical_map, length_char_tuples)
}

pub fn encode_text(text: &str, codes: &HashMap<char, String>, lengths_chars: &[(usize, Vec<char>)]) -> Vec<u8> {
    let mut packed_data = Vec::new();
    let mut buffer = 0u8;
    let mut bits_filled = 0;

    // Serialize lengths and chars
    for (length, chars) in lengths_chars {
        packed_data.push(*length as u8);
        packed_data.push(chars.len() as u8);
        for &char in chars {
            packed_data.push(char as u8);
        }
    }

    // Delimiter to separate metadata and encoded text
    packed_data.push(0);


    for character in text.chars() {
        if let Some(code) = codes.get(&character) {
            for bit in code.chars() {
                buffer <<= 1;
                if bit == '1' {
                    buffer |= 1;
                }
                bits_filled += 1;

                if bits_filled == 8 {
                    packed_data.push(buffer);
                    buffer = 0;
                    bits_filled = 0;
                }
            }
        }
    }
    if bits_filled > 0 {
        buffer <<= 8 - bits_filled;
        packed_data.push(buffer);
    }
    packed_data
}
