use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub parts: Vec<String>,
    pub delimiter: char,
}

impl Namespace {
    pub const DELIMITER: char = '.';
    
    pub fn new(parts: Vec<String>) -> Self {
        Namespace {
            parts,
            delimiter: Self::DELIMITER,
        }
    }
    
    pub fn from_string(s: &str) -> Self {
        Self::from_str_with_delimiter(s, Self::DELIMITER)
    }
    
    pub fn from_str_with_delimiter(s: &str, delimiter: char) -> Self {
        Namespace {
            parts: s.split(delimiter).map(|s| s.to_string()).collect(),
            delimiter,
        }
    }
}

impl FromStr for Namespace {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Namespace::from_string(s))
    }
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parts.join(&self.delimiter.to_string()))
    }
}