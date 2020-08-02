use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap};

pub mod node;
use node::Node;
use node::NodeType::{Character, Joint};

#[derive(Serialize, Deserialize)]
pub struct HuffmanTree {
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

pub fn fill_code_table(code_table: &mut HashMap<u32, String>, tree: &HuffmanTree) {
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

pub fn char_freq(contents: &String) -> HashMap<u32, u32> {
    let mut freq_map: HashMap<u32, u32> = HashMap::new();

    for c in contents.chars() {
        let count = freq_map.entry(c as u32).or_insert(0);
        *count += 1;
    }

    freq_map
}
