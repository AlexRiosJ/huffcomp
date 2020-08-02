use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::{char, usize};

mod huffman_tree;
use huffman_tree::node::NodeType::Character;
use huffman_tree::HuffmanTree;
pub mod config;
use config::Config;

pub fn run(config: Config) -> Result<(), String> {
    match &config.flag[..] {
        "-c" => Ok(compress(config.filename).unwrap()),
        "-d" => Ok(decompress(config.filename).unwrap()),
        _ => {
            let error_message = format!("Found argument '{}' which wasn't expected\n\nUSAGE:\n\thuffcomp [OPTION] [FILENAME]\n\n", config.flag);
            return Err(error_message);
        }
    }
}

fn compress(filename: String) -> Result<(), Box<dyn Error>> {
    println!("Compressing '{}'. . .", filename);
    let contents = fs::read_to_string(&filename)?;

    // Generate characters' frequency map with contents.
    let freq_map = huffman_tree::char_freq(&contents);

    // Create tree with characters' frequency map.
    let tree = huffman_tree::HuffmanTree::new(&freq_map);
    // tree._print();

    // Serialize HuffmanTree struct.
    let tree_bytes: Vec<u8> = bincode::serialize(&tree)?;
    let tree_size = tree_bytes.len().to_be_bytes();

    // Create output file.
    let output_filename = format!("{}.huff", &filename);
    let mut output_file = File::create(&output_filename)?;

    // Write HuffmanTree byte len and HuffmanTree bytes.
    output_file.write_all(&tree_size)?;
    output_file.write_all(&tree_bytes)?;

    // Create and fill the code table map.
    let mut code_table: HashMap<u32, String> = HashMap::new();
    huffman_tree::fill_code_table(&mut code_table, &tree);

    // Generate the encoded string.
    let mut encoded_string = String::from("");
    for c in contents.chars() {
        encoded_string.push_str(code_table.get(&(c as u32)).unwrap());
    }

    // Write encoded string bits length.
    output_file.write(&encoded_string.len().to_be_bytes())?;

    while encoded_string.len() % 8 != 0 {
        encoded_string.push_str("0");
    }

    let mut encoded_bytes: Vec<u8> = "".bytes().collect();
    for _ in 0..encoded_string.len() / 8 {
        encoded_bytes.push(0);
    }

    // Save all the bits into a bytes vector and write to output file.
    for (index, c) in encoded_string.char_indices() {
        encoded_bytes[index / 8] <<= 1;
        encoded_bytes[index / 8] += c as u8 - '0' as u8;
    }
    output_file.write(&encoded_bytes)?;

    println!("Compression finished!");
    println!("Output file: {}", output_filename);

    Ok(())
}

fn decompress(filename: String) -> Result<(), Box<dyn Error>> {
    println!("Decompressing '{}'. . .", filename);
    let encoded = fs::read(&filename)?;
    let mut tree_size: [u8; 8] = [0; 8];

    for i in 0..8 {
        tree_size[i] = encoded[i];
    }

    let tree_size_value = usize::from_be_bytes(tree_size);
    let tree_encoded = &encoded[8..(tree_size_value + 8)];
    let tree: HuffmanTree = bincode::deserialize(tree_encoded)?;
    let mut node = tree.get_root();

    let mut bits_to_decode: [u8; 8] = [0; 8];
    for i in 0..8 {
        bits_to_decode[i] = encoded[i + (tree_size_value + 8)];
    }
    let bits_to_decode = usize::from_be_bytes(bits_to_decode);

    let bytes_encoded = &encoded[(tree_size_value + 16)..];

    let output_filename = format!("{}d.txt", &filename);
    let mut output_file = File::create(&output_filename)?;
    let mut output_string = String::from("");

    let mut bit_counter = 0;
    for byte in bytes_encoded {
        for i in 0..8 {
            let mask = 0x80 >> i;
            let bit = (mask & byte) >> (7 - i);

            if let (Some(left), Some(right)) = (&node.left, &node.right) {
                node = if bit == 1 { &*right } else { &*left };
                if let None = node.left {
                    if let Character(character) = node.value {
                        let chars = char::from_u32(character).unwrap();
                        output_string.push(chars);
                    }
                    node = tree.get_root();
                }
            }

            bit_counter += 1;
            if bit_counter == bits_to_decode {
                break;
            }
        }
    }

    output_file.write(output_string.as_bytes())?;

    println!("Decompression finished!");
    println!("Output file: {}", output_filename);

    Ok(())
}
