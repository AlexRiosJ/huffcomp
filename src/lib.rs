use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::{char, usize};

use NodeType::{Character, Joint};

pub struct Config {
    flag: String,
    filename: String,
}

#[derive(Serialize, Deserialize)]
struct HuffmanTree {
    root: Node,
}

impl HuffmanTree {
    pub fn new(freq_map: &HashMap<u32, u32>) -> HuffmanTree {
        let mut queue = BinaryHeap::new();

        for (ch, freq) in freq_map {
            let new_node = Node::new(Character(*ch as u32), *freq);
            queue.push(new_node);
        }

        let mut left_node: Node;
        let mut right_node: Node;
        let mut joint_node: Node;

        while queue.len() >= 2 {
            left_node = queue.pop().unwrap();
            right_node = queue.pop().unwrap();

            joint_node = Node::new(Joint, left_node.frequency + right_node.frequency);
            joint_node.left = Some(Box::new(left_node));
            joint_node.right = Some(Box::new(right_node));

            queue.push(joint_node);
        }

        HuffmanTree {
            root: queue.pop().unwrap(),
        }
    }

    pub fn _print(&self) {
        println!("-------------Printing Tree--------------");
        self._print_recursive(&self.root, 0);
        println!("----------------------------------------");
    }

    fn _print_recursive(&self, node: &Node, spaces: u32) {
        let mut temp = String::from("");
        for _ in 0..spaces {
            temp.push_str("|  ");
        }
        println!("{}{}", temp, node);

        if let (Some(left), Some(right)) = (&node.left, &node.right) {
            self._print_recursive(&*left, spaces + 1);
            self._print_recursive(&*right, spaces + 1);
        }
    }

    pub fn get_root(&self) -> &Node {
        &self.root
    }
}

#[derive(Eq, Serialize, Deserialize)]
struct Node {
    value: NodeType,
    frequency: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: NodeType, frequency: u32) -> Node {
        Node {
            value: match value {
                Character(c) => Character(c),
                Joint => Joint,
            },
            frequency,
            left: None,
            right: None,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            Character(c) => match char::from_u32(c).unwrap() {
                '\n' => write!(f, "[\\n, {}]", self.frequency),
                '\r' => write!(f, "[\\r, {}]", self.frequency),
                '\0' => write!(f, "[\\0, {}]", self.frequency),
                _ => write!(f, "[{}, {}]", char::from_u32(c).unwrap(), self.frequency),
            },
            Joint => write!(f, "[{}]", self.frequency),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
enum NodeType {
    Character(u32),
    Joint,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let flag = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a configuration flag."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file name."),
        };

        Ok(Config { flag, filename })
    }
}

pub fn run(config: Config) -> Result<(), String> {
    match &config.flag[..] {
        "-c" => Ok(compress(config.filename).unwrap()),
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
    let freq_map = char_freq(&contents);

    // Create tree with characters' frequency map.
    let tree = HuffmanTree::new(&freq_map);
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
    fill_code_table(&mut code_table, &tree);

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
    let mut counter = 0;
    let mut mask_count = 0;
    for c in encoded_string.chars() {
        encoded_bytes[counter / 8] += c.to_digit(2).unwrap() as u8;
        if mask_count < 7 {
            encoded_bytes[counter / 8] <<= 1;
            mask_count += 1;
        } else {
            mask_count = 0;
        }
        counter += 1;
    }
    output_file.write(&encoded_bytes)?;

    println!("Compression finished!");
    println!("Output file: {}", output_filename);

    Ok(())
}

fn char_freq(contents: &String) -> HashMap<u32, u32> {
    let mut freq_map: HashMap<u32, u32> = HashMap::new();

    for c in contents.chars() {
        let count = freq_map.entry(c as u32).or_insert(0);
        *count += 1;
    }

    freq_map
}

fn fill_code_table(code_table: &mut HashMap<u32, String>, tree: &HuffmanTree) {
    fill_code_table_recursive(code_table, &tree.get_root(), String::from("")).unwrap();
}

fn fill_code_table_recursive<'a>(
    code_table: &'a mut HashMap<u32, String>,
    node: &Node,
    mask: String,
) -> Result<(), &'static str> {
    if let None = &node.left {
        let character: u32;
        if let Character(c) = &node.value {
            character = *c;
        } else {
            return Err("Something went wrong with the table creation");
        }
        code_table.insert(character, mask);
        return Ok(());
    }

    Ok(
        if let (Some(left), Some(right)) = (&node.left, &node.right) {
            fill_code_table_recursive(code_table, &*left, format!("{}0", mask))?;
            fill_code_table_recursive(code_table, &*right, format!("{}1", mask))?;
        },
    )
}

// Decompressing Tree
// let encoded = fs::read(&output_filename)?;
// let mut tree_size_output: [u8; 8] = [0; 8];

// for i in 0..8 {
//     tree_size_output[i] = encoded[i];
// }

// let tree_size_value = usize::from_be_bytes(tree_size_output);
// let tree_encoded = &encoded[8..(tree_size_value + 8)];

// let tree_output: HuffmanTree = bincode::deserialize(tree_encoded)?;
// tree_output._print();

// let mut code_table: HashMap<u32, String> = HashMap::new();
// fill_code_table(&mut code_table, &tree);

// let mut encoded_string = String::from("");

// for c in contents.chars() {
//     encoded_string.push_str(code_table.get(&(c as u32)).unwrap());
// }
