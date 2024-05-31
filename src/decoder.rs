use std::collections::HashMap;

fn build_decoding_map_from_metadata(metadata: &[u8]) -> HashMap<String, char> {
  let mut decoding_map = HashMap::new();
  let mut i = 0;
  let mut c_code = 0;

  while metadata[i] != 0 {
      let length = metadata[i] as usize;
      let num_chars = metadata[i + 1] as usize;
      i += 2;

      for _ in 0..num_chars {
          let char = metadata[i] as char;
          i += 1;
          let code = format!("{:0length$b}", c_code, length = length);
          decoding_map.insert(code, char);
          c_code += 1;
      }
      c_code <<= 1;
  }

  decoding_map
}

pub fn decode_text(encoded_data: &[u8]) -> String {
  // Extract metadata
  let mut meta_end = 0;
  while encoded_data[meta_end] != 0 {
      meta_end += 1;
  }
  meta_end += 1;  // Skip the delimiter

  let decoding_map = build_decoding_map_from_metadata(&encoded_data[..meta_end]);

  let mut current_code = String::new();
  let mut decoded_text = String::new();

  for &byte in &encoded_data[meta_end..] {
      for i in (0..8).rev() {
          let bit = (byte >> i) & 1;
          current_code.push(if bit == 1 { '1' } else { '0' });

          if let Some(&char) = decoding_map.get(&current_code) {
              decoded_text.push(char);
              current_code.clear();
          }
      }
  }

  decoded_text
}
