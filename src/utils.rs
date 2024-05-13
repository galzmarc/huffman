use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HuffmanNode {
    symbol: Option<char>,
    freq: usize,
    left: Box<Option<HuffmanNode>>,
    right: Box<Option<HuffmanNode>>,
}

impl HuffmanNode {
    pub fn new(freq_map: &HashMap<char, usize>) -> Self {
        // Build the priority queue
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
}

pub fn analyze_frequency(contents: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();

    for character in contents.chars() {
        let count = freq_map.entry(character).or_insert(0);
        *count += 1;
    }

    freq_map
}
