use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DecodeError {
    pub msg: String,
}

impl DecodeError {
    pub fn new(msg: &str) -> DecodeError {
        DecodeError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for DecodeError {}
