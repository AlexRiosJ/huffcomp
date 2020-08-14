use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{char, usize};

mod huffman_tree;
use huffman_tree::node::NodeType::Character;
use huffman_tree::HuffmanTree;
pub mod config;
use config::Config;
mod errors;
use errors::InputError;

pub fn run(config: Config) -> Result<(), String> {
    match &config.flag[..] {
        "-c" | "--compress" => {
            if let Err(err) = compress(config.filename) {
                return Err(err.to_string());
            }
            Ok(())
        }
        "-d" | "--decompress" => {
            if let Err(err) = decompress(config.filename) {
                return Err(err.to_string());
            }
            Ok(())
        }
        "-V" | "--version" => Ok(version_message()),
        "-h" | "--help" | "" => Ok(help_message()),
        _ => {
            let error_message = format!("\n\tFound argument '{}' which wasn't expected\n\nSee 'huffcomp --help' for more information.\n", config.flag);
            return Err(error_message);
        }
    }
}

fn compress(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&filename)?;

    if contents.len() == 0 {
        let error_message = format!("File must not be empty");
        return Err(Box::new(InputError(error_message)));
    }

    // Generate characters' frequency map with contents.
    let freq_map = huffman_tree::char_freq(&contents);

    // Create tree with characters' frequency map.
    let tree = HuffmanTree::new(&freq_map);
    // tree._print();

    // Serialize HuffmanTree struct.
    let tree_bytes: Vec<u8> = bincode::serialize(&tree)?;
    let tree_size = tree_bytes.len().to_be_bytes();

    // Create output file.
    let output_filename = format!("{}.huff", &filename);
    let mut output_file = File::create(&output_filename)?;

    println!("Compressing '{}'. . .", filename);

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
    let filename_extension = Path::new(&filename).extension();

    match filename_extension {
        Some(ext) => {
            if ext != "huff" {
                let error_message = format!("File must have the correct extension.\n\tExpected:\t\"huff\"\n\tFound:\t\t{:?}\n", ext);
                return Err(Box::new(InputError(error_message)));
            }
        }
        None => {
            let error_message = String::from("File must have \"huff\" extension.\n");
            return Err(Box::new(InputError(error_message)));
        }
    }

    let encoded = fs::read(&filename)?;

    if encoded.len() == 0 {
        let error_message = format!("File must not be empty");
        return Err(Box::new(InputError(error_message)));
    }

    let mut tree_size: [u8; 8] = [0; 8];

    for i in 0..8 {
        tree_size[i] = encoded[i];
    }

    let tree_size_value = usize::from_be_bytes(tree_size);
    let tree_encoded = &encoded[8..(tree_size_value + 8)];
    let tree: HuffmanTree = bincode::deserialize(tree_encoded)?;
    let mut node = tree.get_root();

    println!("Decompressing '{}'. . .", filename);

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

fn help_message() {
    println!("Huffman coding program for compression and decompression of text files");
    println!();
    println!("USAGE:");
    println!("\thuffcomp [OPTIONS] [FILENAME]");
    println!();
    println!("OPTIONS:");
    println!("\t-c, --compress\t\tCompress the given text file");
    println!("\t-d, --decompress\tDecompress a valid .huff file");
    println!("\t-V, --version\t\tPrint version info and exit");
    println!();
}

fn version_message() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    println!("{} {}", NAME, VERSION);
}
