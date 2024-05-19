use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct HuffmanNode {
    symbol: Option<char>,
    freq: usize,
    left: Box<Option<HuffmanNode>>,
    right: Box<Option<HuffmanNode>>,
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the ordering for min-heap behavior
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HuffmanNode {
    pub fn build_tree(freq_map: &HashMap<char, usize>) -> Self {
        // Build a priority queue
        let mut priority_queue: BinaryHeap<HuffmanNode> = BinaryHeap::with_capacity(freq_map.len());
        for (&symbol, &freq) in freq_map {
            priority_queue.push(HuffmanNode {
                symbol: Some(symbol),
                freq,
                left: Box::new(None),
                right: Box::new(None),
            })
        }

        // Iterate through the priority queue to build the Huffman Tree
        while priority_queue.len() > 1 {
            let left_child = priority_queue.pop().unwrap();
            let right_child = priority_queue.pop().unwrap();

            let merged = HuffmanNode {
                symbol: None,
                freq: left_child.freq + right_child.freq,
                left: Box::new(Some(left_child)),
                right: Box::new(Some(right_child)),
            };

            priority_queue.push(merged);
        }

        // Create the root of the Huffman Tree
        let root = priority_queue.pop().unwrap();
        root
    }

    pub fn encode(&self, current_code: String, codes: &mut HashMap<char, String>) {
        if let Some(symbol) = self.symbol {
            codes.insert(symbol, current_code);
        } else {
            if let Some(ref left) = *self.left {
                left.encode(format!("{}0", current_code), codes);
            }
            if let Some(ref right) = *self.right {
                right.encode(format!("{}1", current_code), codes);
            }
        }
    }

    pub fn decode(&self, text: &str) -> String {
        let mut decoded_text = String::new();
        let mut current_node = self;
        for bit in text.chars() {
            match bit {
                '0' => {
                    if let Some(ref left) = *current_node.left {
                        current_node = left;
                    }
                }
                '1' => {
                    if let Some(ref right) = *current_node.right {
                        current_node = right;
                    }
                }
                _ => panic!("Invalid bit in encoded string!"),
            }
            // If we reach a leaf node, append the corresponding character and reset to root
            if let Some(symbol) = current_node.symbol {
                decoded_text.push(symbol);
                current_node = self // Reset to root
            }
        }
        decoded_text
    }
}

pub fn analyze_frequency(contents: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();

    for character in contents.chars() {
        let count = freq_map.entry(character).or_insert(0);
        *count += 1;
    }

    freq_map
}

pub fn encode_text(text: &str, codes: &HashMap<char, String>) -> Vec<u8> {
    let mut packed_data = Vec::new();
    let mut buffer = 0u8;
    let mut bits_filled = 0;

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
