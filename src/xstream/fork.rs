// XStream Fork Operations - Channel splitting with macro sugar
// Split token streams by namespace channels

use crate::xstream::types::{TokenBucket, BucketMode};
use std::collections::HashMap;

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
pub fn fork_by_namespace(input: &str, channels: &[&str]) -> HashMap<String, String> {
    let bucket = match TokenBucket::from_str(input, BucketMode::Hybrid) {
        Ok(b) => b,
        Err(_) => return HashMap::new(), // Invalid input returns empty map
    };
    
    let mut result = HashMap::new();
    
    for channel in channels {
        if let Some(namespace_data) = bucket.get_namespace(channel) {
            let tokens: Vec<String> = namespace_data
                .iter()
                .map(|(key, value)| format!("{}:{}=\"{}\"", channel, key, value))
                .collect();
            
            if !tokens.is_empty() {
                result.insert(channel.to_string(), tokens.join("; "));
            }
        }
    }
    
    result
}

/// Fork all namespaces found in the stream
pub fn fork_all_namespaces(input: &str) -> HashMap<String, String> {
    let bucket = match TokenBucket::from_str(input, BucketMode::Hybrid) {
        Ok(b) => b,
        Err(_) => return HashMap::new(),
    };
    
    let mut result = HashMap::new();
    
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
            result.insert(namespace.clone(), tokens.join("; "));
        }
    }
    
    result
}

/// Fork by pattern matching
pub fn fork_by_pattern(input: &str, pattern: &str) -> HashMap<String, String> {
    use regex::Regex;
    
    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(_) => return HashMap::new(), // Invalid regex returns empty
    };
    
    let bucket = match TokenBucket::from_str(input, BucketMode::Hybrid) {
        Ok(b) => b,
        Err(_) => return HashMap::new(),
    };
    
    let mut result = HashMap::new();
    
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
                result.insert(namespace.clone(), tokens.join("; "));
            }
        }
    }
    
    result
}

/// Macro for clean fork syntax
#[macro_export]
macro_rules! fork {
    // fork!(stream, "ch1", "ch2", "ch3") -> (ch1_stream, ch2_stream, ch3_stream)
    ($stream:expr, $($channel:expr),+) => {{
        let channels = vec![$($channel),+];
        let map = $crate::xstream::fork::fork_by_namespace($stream, &channels);
        ($(map.get($channel).cloned().unwrap_or_default()),+)
    }};
}

/// Macro for fork all namespaces
#[macro_export]
macro_rules! fork_all {
    ($stream:expr) => {{
        $crate::xstream::fork::fork_all_namespaces($stream)
    }};
}

/// Macro for fork by pattern  
#[macro_export]
macro_rules! fork_pattern {
    ($stream:expr, $pattern:expr) => {{
        $crate::xstream::fork::fork_by_pattern($stream, $pattern)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fork_by_namespace() {
        let input = "ui:click=\"btn1\"; db:host=\"localhost\"; ui:theme=\"dark\"; log:level=\"info\"";
        let channels = vec!["ui", "db", "log"];
        let result = fork_by_namespace(input, &channels);
        
        assert_eq!(result.len(), 3);
        assert!(result["ui"].contains("ui:click=\"btn1\""));
        assert!(result["ui"].contains("ui:theme=\"dark\""));
        assert!(result["db"].contains("db:host=\"localhost\""));
        assert!(result["log"].contains("log:level=\"info\""));
    }

    #[test] 
    fn test_fork_macro() {
        let input = "ui:click=\"btn1\"; db:host=\"localhost\"; log:level=\"info\"";
        let (ui, db, log) = fork!(input, "ui", "db", "log");
        
        assert!(ui.contains("ui:click=\"btn1\""));
        assert!(db.contains("db:host=\"localhost\""));
        assert!(log.contains("log:level=\"info\""));
    }

    #[test]
    fn test_fork_all() {
        let input = "ui:click=\"btn1\"; db:host=\"localhost\"; log:level=\"info\"";
        let channels = fork_all!(input);
        
        assert!(channels.contains_key("ui"));
        assert!(channels.contains_key("db"));
        assert!(channels.contains_key("log"));
    }

    #[test]
    fn test_fork_pattern() {
        let input = "api1:endpoint=\"/users\"; api2:endpoint=\"/posts\"; db:host=\"localhost\"";
        let apis = fork_pattern!(input, r"api\d+");
        
        assert!(apis.contains_key("api1"));
        assert!(apis.contains_key("api2"));
        assert!(!apis.contains_key("db"));
    }
}