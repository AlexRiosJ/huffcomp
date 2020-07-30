use std::char;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fmt;
use std::fs;

use NodeType::{Character, Joint};

pub struct Config {
    flag: String,
    filename: String,
}

struct HuffmanTree {
    root: Node,
}

impl HuffmanTree {
    pub fn new(freq_map: &HashMap<char, u32>) -> HuffmanTree {
        let mut queue = BinaryHeap::new();

        for (ch, freq) in freq_map {
            let new_node = Node::new(Character(*ch), *freq);
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

    pub fn print(&self) {
        println!("-------------Printing Tree--------------");
        self.print_recursive(&self.root, 0);
        println!("----------------------------------------");
    }

    pub fn print_recursive(&self, node: &Node, spaces: u32) {
        let mut temp = String::from("");
        for _ in 0..spaces {
            temp.push_str("|  ");
        }
        println!("{}{}", temp, node);

        if let (Some(left), Some(right)) = (&node.left, &node.right) {
            self.print_recursive(&*left, spaces + 1);
            self.print_recursive(&*right, spaces + 1);
        }
    }

    pub fn get_root(&self) -> &Node {
        &self.root
    }
}

#[derive(Eq)]
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
            Character(c) => match c {
                '\n' => write!(f, "[\\n, {}]", self.frequency),
                '\r' => write!(f, "[\\r, {}]", self.frequency),
                '\0' => write!(f, "[\\0, {}]", self.frequency),
                _ => write!(f, "[{}, {}]", c, self.frequency),
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

#[derive(Eq, PartialEq, Debug)]
enum NodeType {
    Character(char),
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
    let contents = fs::read_to_string(filename)?;
    // println!("{}", contents.len());

    let freq_map = char_freq(&contents);

    let tree = HuffmanTree::new(&freq_map);
    tree.print();

    let mut code_table: HashMap<char, String> = HashMap::new();
    fill_code_table(&mut code_table, &tree);

    let mut encoded_string = String::from("");

    for c in contents.chars() {
        encoded_string.push_str(code_table.get(&c).unwrap());
    }

    for _ in 0..encoded_string.len() % 8 {
        encoded_string.push_str("0");
    }

    println!("{}", encoded_string.len() / 8);
    println!("{}", encoded_string);

    Ok(())
}

fn char_freq(contents: &String) -> HashMap<char, u32> {
    let mut freq_map: HashMap<char, u32> = HashMap::new();

    for c in contents.chars() {
        let count = freq_map.entry(c).or_insert(0);
        *count += 1;
    }

    freq_map
}

fn fill_code_table(code_table: &mut HashMap<char, String>, tree: &HuffmanTree) {
    fill_code_table_recursive(code_table, &tree.get_root(), String::from("")).unwrap();
}

fn fill_code_table_recursive<'a>(
    code_table: &'a mut HashMap<char, String>,
    node: &Node,
    mask: String,
) -> Result<(), &'static str> {
    if let None = &node.left {
        let character: char;
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
