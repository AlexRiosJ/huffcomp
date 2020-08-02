use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::{char, usize};

use NodeType::{Character, Joint};

#[derive(Eq, Serialize, Deserialize)]
pub struct Node {
    pub value: NodeType,
    pub frequency: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: NodeType, frequency: u32) -> Node {
        Node {
            value,
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
pub enum NodeType {
    Character(u32),
    Joint,
}
