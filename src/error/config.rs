use std::fmt;

#[derive(Debug, Clone)]
pub struct SetConfigError {
    pub num: i32
}

impl fmt::Display for SetConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to set the config to: {}. Try checking if that config exists!", self.num)
    }
}

#[derive(Debug, Clone)]
pub struct SetConfigArgumentError {
    pub problem_word: String
}

impl fmt::Display for SetConfigArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\" is not a number.", self.problem_word)
    }
}

