use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InputError(pub String);

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Input error: {}", self.0)
    }
}

impl Error for InputError {}
