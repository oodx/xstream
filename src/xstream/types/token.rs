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

fn strip_quotes(s: &str) -> String {
    let s = s.trim();
    if s.len() >= 2 {
        if (s.starts_with('"') && s.ends_with('"')) || 
           (s.starts_with('\'') && s.ends_with('\'')) {
            s[1..s.len()-1].to_string()
        } else {
            s.to_string()
        }
    } else {
        s.to_string()
    }
}

// Standalone tokenization function that can be reused
pub fn tokenize_string(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    
    for token_str in input.split(';') {
        // Only trim leading spaces (allow space after ;) but not trailing spaces (no space before ;)
        let token_str = token_str.trim_start();
        if token_str.is_empty() { continue; }
        
        // Check for trailing spaces (space before ;)
        if token_str != token_str.trim_end() {
            return Err(format!("Malformed token '{}': trailing spaces not allowed", token_str.trim_end()));
        }
        
        // Split on first '='
        let (key_part, value_part) = match token_str.split_once('=') {
            Some((k, v)) => (k, v),
            None => {
                // More specific error for malformed tokens
                if !token_str.trim().is_empty() {
                    return Err(format!("Malformed token '{}': missing '=' separator", token_str));
                }
                continue;
            },
        };
        
        // Check for spaces around '=' - key should not have trailing spaces, value should not have leading spaces
        if key_part != key_part.trim_end() {
            return Err(format!("Malformed token '{}': space before '=' not allowed", token_str));
        }
        if value_part != value_part.trim_start() {
            return Err(format!("Malformed token '{}': space after '=' not allowed", token_str));
        }
        
        let key_part = key_part.trim(); // Allow leading spaces in key for consistency
        let value = strip_quotes(value_part);
        
        // Check for empty key
        if key_part.is_empty() {
            return Err(format!("Malformed token '{}': empty key", token_str));
        }
        
        // Check for namespace separator ':'
        let (namespace, key) = match key_part.split_once(':') {
            Some((ns, k)) => {
                // Validate namespace - no spaces allowed
                if ns.contains(' ') {
                    return Err(format!("Malformed token '{}': spaces not allowed in namespace '{}'", token_str, ns));
                }
                // Validate key part - no spaces allowed
                if k.contains(' ') {
                    return Err(format!("Malformed token '{}': spaces not allowed in key '{}'", token_str, k));
                }
                // Parse namespace with its internal delimiter
                let namespace = Namespace::from_string(ns);
                (Some(namespace), k.to_string())
            },
            None => {
                // Even non-prefixed keys shouldn't have spaces
                if key_part.contains(' ') {
                    return Err(format!("Malformed token '{}': spaces not allowed in key '{}'", token_str, key_part));
                }
                (None, key_part.to_string())
            },
        };
        
        tokens.push(Token { namespace, key, value });
    }
    
    Ok(tokens)
}

// Helper function to validate if a string can be tokenized
pub fn is_token_streamable(input: &str) -> bool {
    tokenize_string(input).is_ok()
}

impl TokenStreamable for str {
    fn tokenize(&self) -> Result<Vec<Token>, String> {
        tokenize_string(self)
    }
    
    fn validate(&self) -> Result<(), String> {
        tokenize_string(self).map(|_| ())
    }
}