use std::fmt;

pub mod config;

#[derive(Debug, Clone)]
pub struct Error {
    pub cause: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}
