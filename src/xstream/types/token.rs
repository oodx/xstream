use std::fmt;
use std::str::FromStr;
use super::namespace::Namespace;

#[derive(Debug, Clone)]
pub struct Token {
    pub namespace: Option<Namespace>,
    pub key: String,
    pub value: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.namespace {
            Some(ns) => write!(f, "{}:{}={}", ns, self.key, self.value),
            None => write!(f, "{}={}", self.key, self.value),
        }
    }
}

impl FromStr for Token {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key_part, value) = s.split_once('=')
            .ok_or_else(|| "Token must contain '='".to_string())?;
        
        let (namespace, key) = match key_part.split_once(':') {
            Some((ns, k)) => (Some(Namespace::from_string(ns)), k.to_string()),
            None => (None, key_part.to_string()),
        };
        
        Ok(Token {
            namespace,
            key,
            value: value.to_string(),
        })
    }
}

pub trait TokenStreamable {
    fn tokenize(&self) -> Result<Vec<Token>, String>;
    fn validate(&self) -> Result<(), String>;
}

impl TokenStreamable for str {
    fn tokenize(&self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        for token_str in self.split(';') {
            let token_str = token_str.trim();
            if token_str.is_empty() { continue; }
            
            // Split on first '='
            let (key_part, value) = match token_str.split_once('=') {
                Some((k, v)) => (k, v.to_string()),
                None => continue,
            };
            
            // Check for namespace separator ':'
            let (namespace, key) = match key_part.split_once(':') {
                Some((ns, k)) => {
                    // Parse namespace with its internal delimiter
                    let namespace = Namespace::from_string(ns);
                    (Some(namespace), k.to_string())
                },
                None => (None, key_part.to_string()),
            };
            
            tokens.push(Token { namespace, key, value });
        }
        
        Ok(tokens)
    }
    
    fn validate(&self) -> Result<(), String> {
        self.tokenize().map(|_| ())
    }
}