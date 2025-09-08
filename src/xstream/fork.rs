// XStream REAL Fork Operations - Complete RSB Streamable Implementation
// Consolidating all features from fake fork.rs with proper RSB patterns

use rsb::prelude::*;
use crate::xstream::types::{TokenBucket, BucketMode};

/// Fork operation that splits a stream by namespace
pub struct Fork;

impl Streamable for Fork {
    type Args = Vec<String>; // Channel names to fork into
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        fork_by_namespace(stdin, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>())
    }
}

/// Fork all namespaces operation
pub struct ForkAll;

impl Streamable for ForkAll {
    type Args = (); // No args needed
    
    fn stream_apply(stdin: &str, _args: Self::Args) -> String {
        fork_all_namespaces(stdin)
    }
}

/// Fork by pattern operation
pub struct ForkPattern;

impl Streamable for ForkPattern {
    type Args = String; // Regex pattern
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        fork_by_pattern(stdin, &args)
    }
}

// Core fork functions from fake version (now using RSB compatible format)

/// Fork modes for different splitting strategies
#[derive(Debug, Clone)]
pub enum ForkMode {
    /// Match exact namespace names
    Exact,
    /// Match namespaces under a prefix (e.g., "api" matches "api.v1", "api.v2")
    Under(String),
    /// Match using regex pattern
    Regex(String),
}

/// Fork a token stream by namespace into separate channel streams
pub fn fork_by_namespace(input: &str, channels: &[&str]) -> String {
    let bucket = match TokenBucket::from_str(input, BucketMode::Hybrid) {
        Ok(b) => b,
        Err(_) => return String::new(), // Invalid input returns empty
    };
    
    let mut results = Vec::new();
    
    for channel in channels {
        if let Some(namespace_data) = bucket.get_namespace(channel) {
            let tokens: Vec<String> = namespace_data
                .iter()
                .map(|(key, value)| format!("{}:{}=\"{}\"", channel, key, value))
                .collect();
            
            if !tokens.is_empty() {
                results.push(format!("{}: {}", channel, tokens.join("; ")));
            }
        }
    }
    
    results.join("\n")
}

/// Fork all namespaces found in the stream
pub fn fork_all_namespaces(input: &str) -> String {
    let bucket = match TokenBucket::from_str(input, BucketMode::Hybrid) {
        Ok(b) => b,
        Err(_) => return String::new(),
    };
    
    let mut results = Vec::new();
    
    for (namespace, data) in &bucket.data {
        let tokens: Vec<String> = data
            .iter()
            .map(|(key, value)| {
                if namespace == "global" {
                    format!("{}=\"{}\"", key, value)
                } else {
                    format!("{}:{}=\"{}\"", namespace, key, value)
                }
            })
            .collect();
        
        if !tokens.is_empty() {
            results.push(format!("{}: {}", namespace, tokens.join("; ")));
        }
    }
    
    results.join("\n")
}

/// Fork by pattern matching
pub fn fork_by_pattern(input: &str, pattern: &str) -> String {
    use regex::Regex;
    
    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(_) => return String::new(), // Invalid regex returns empty
    };
    
    let bucket = match TokenBucket::from_str(input, BucketMode::Hybrid) {
        Ok(b) => b,
        Err(_) => return String::new(),
    };
    
    let mut results = Vec::new();
    
    for (namespace, data) in &bucket.data {
        if re.is_match(namespace) {
            let tokens: Vec<String> = data
                .iter()
                .map(|(key, value)| {
                    if namespace == "global" {
                        format!("{}=\"{}\"", key, value)
                    } else {
                        format!("{}:{}=\"{}\"", namespace, key, value)
                    }
                })
                .collect();
            
            if !tokens.is_empty() {
                results.push(format!("{}: {}", namespace, tokens.join("; ")));
            }
        }
    }
    
    results.join("\n")
}

/// Macro for clean fork syntax - RSB compatible
#[macro_export]
macro_rules! fork {
    // fork!(stream, "ch1", "ch2", "ch3") -> HashMap result
    ($stream:expr, $($channel:expr),+) => {{
        let channels = vec![$($channel),+];
        let result = $crate::xstream::fork::fork_by_namespace($stream, &channels);
        
        // Convert to HashMap for compatibility
        let mut map = std::collections::HashMap::new();
        for line in result.lines() {
            if let Some((channel, tokens)) = line.split_once(": ") {
                map.insert(channel.to_string(), tokens.to_string());
            }
        }
        map
    }};
}

/// Macro for fork all namespaces  
#[macro_export]
macro_rules! fork_all {
    ($stream:expr) => {{
        let result = $crate::xstream::fork::fork_all_namespaces($stream);
        
        // Convert to HashMap for compatibility
        let mut map = std::collections::HashMap::new();
        for line in result.lines() {
            if let Some((namespace, tokens)) = line.split_once(": ") {
                map.insert(namespace.to_string(), tokens.to_string());
            }
        }
        map
    }};
}

/// Macro for fork by pattern
#[macro_export]
macro_rules! fork_pattern {
    ($stream:expr, $pattern:expr) => {{
        let result = $crate::xstream::fork::fork_by_pattern($stream, $pattern);
        
        // Convert to HashMap for compatibility  
        let mut map = std::collections::HashMap::new();
        for line in result.lines() {
            if let Some((namespace, tokens)) = line.split_once(": ") {
                map.insert(namespace.to_string(), tokens.to_string());
            }
        }
        map
    }};
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fork_by_namespace() {
        let input = "ui:click=\"btn1\"; db:host=\"localhost\"; ui:theme=\"dark\"; log:level=\"info\"";
        let result = fork_by_namespace(input, &["ui", "db", "log"]);
        
        // Check that UI namespace line contains both expected tokens (order doesn't matter)
        assert!(result.contains("ui:"));
        assert!(result.contains("ui:click=\"btn1\""));
        assert!(result.contains("ui:theme=\"dark\""));
        
        // Check other namespaces
        assert!(result.contains("db: db:host=\"localhost\""));
        assert!(result.contains("log: log:level=\"info\""));
    }

    #[test] 
    fn test_fork_macro() {
        let input = "ui:click=\"btn1\"; db:host=\"localhost\"; log:level=\"info\"";
        let result = fork!(input, "ui", "db", "log");
        
        assert!(result.contains_key("ui"));
        assert!(result.contains_key("db"));
        assert!(result.contains_key("log"));
        assert!(result["ui"].contains("ui:click=\"btn1\""));
        assert!(result["db"].contains("db:host=\"localhost\""));
    }

    #[test]
    fn test_fork_all() {
        let input = "ui:click=\"btn1\"; db:host=\"localhost\"; log:level=\"info\"";
        let result = fork_all!(input);
        
        assert!(result.contains_key("ui"));
        assert!(result.contains_key("db"));
        assert!(result.contains_key("log"));
    }

    #[test]
    fn test_fork_pattern() {
        let input = "api1:endpoint=\"/users\"; api2:endpoint=\"/posts\"; db:host=\"localhost\"";
        let result = fork_pattern!(input, r"api\d+");  // Fixed regex pattern
        
        assert!(result.contains_key("api1"));
        assert!(result.contains_key("api2"));
        assert!(!result.contains_key("db"));
    }
    
    #[test]
    fn test_real_fork_with_streamable() {
        let input = "ui:click=\"btn1\"; db:host=\"localhost\"; ui:theme=\"dark\"".to_string();
        
        // Fork using Streamable trait
        let forked = input.stream_apply(Fork, vec!["ui".to_string(), "db".to_string()]);
        
        // Verify fork splits by namespace - order within namespace may vary due to HashMap
        assert!(forked.contains("ui:") && forked.contains("ui:click=\"btn1\"") && forked.contains("ui:theme=\"dark\""));
        assert!(forked.contains("db: db:host=\"localhost\""));
    }
    
    #[test]
    fn test_fork_all_streamable() {
        let input = "ui:click=\"btn1\"; db:host=\"localhost\"; log:level=\"info\"".to_string();
        
        let result = input.stream_apply(ForkAll, ());
        
        assert!(result.contains("ui:"));
        assert!(result.contains("db:"));
        assert!(result.contains("log:"));
    }
    
    #[test]
    fn test_fork_pattern_streamable() {
        let input = "api1:endpoint=\"/users\"; api2:endpoint=\"/posts\"; db:host=\"localhost\"".to_string();
        
        let result = input.stream_apply(ForkPattern, r"api\d+".to_string());  // Fixed regex pattern
        
        assert!(result.contains("api1:"));
        assert!(result.contains("api2:"));
        assert!(!result.contains("db:"));
    }
}