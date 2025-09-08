// XStream Merge Operations - Channel combining with macro sugar
// Combine multiple token streams with various strategies

use crate::xstream::types::is_token_streamable;
use std::collections::HashMap;

/// Merge strategies for combining multiple streams
#[derive(Debug, Clone)]
pub enum MergeStrategy {
    /// Simple concatenation in order given
    Concat,
    /// Interleave tokens in round-robin fashion  
    Interleave,
    /// Priority order - channels listed first get precedence
    Priority(Vec<String>),
    /// Sort tokens by key name
    Sort,
}

/// Collision handling for duplicate keys across streams
#[derive(Debug, Clone)]
pub enum CollisionPolicy {
    /// Keep first occurrence of duplicate key
    KeepFirst,
    /// Keep last occurrence of duplicate key  
    KeepLast,
    /// Annotate duplicates with dupe:key=true
    Annotate,
}

/// Basic merge - concatenate streams in order
pub fn merge_concat(streams: &[&str]) -> String {
    let valid_streams: Vec<&str> = streams
        .iter()
        .filter(|s| is_token_streamable(s))
        .copied()
        .collect();
    
    if valid_streams.is_empty() {
        return String::new();
    }
    
    valid_streams.join("; ")
}

/// Merge with strategy
pub fn merge_with_strategy(streams: &[&str], strategy: MergeStrategy) -> String {
    match strategy {
        MergeStrategy::Concat => merge_concat(streams),
        MergeStrategy::Interleave => merge_interleave(streams),
        MergeStrategy::Priority(priorities) => merge_priority(streams, &priorities),
        MergeStrategy::Sort => merge_sorted(streams),
    }
}

/// Interleave tokens from multiple streams in round-robin
pub fn merge_interleave(streams: &[&str]) -> String {
    let token_lists: Vec<Vec<&str>> = streams
        .iter()
        .filter(|s| is_token_streamable(s))
        .map(|s| s.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect())
        .collect();
    
    if token_lists.is_empty() {
        return String::new();
    }
    
    let mut result = Vec::new();
    let max_len = token_lists.iter().map(|list| list.len()).max().unwrap_or(0);
    
    for i in 0..max_len {
        for list in &token_lists {
            if i < list.len() {
                result.push(list[i]);
            }
        }
    }
    
    result.join("; ")
}

/// Merge with priority ordering
pub fn merge_priority(streams: &[&str], priorities: &[String]) -> String {
    // Parse streams into namespace -> tokens map
    let mut namespace_streams: HashMap<String, Vec<String>> = HashMap::new();
    
    for stream in streams {
        if !is_token_streamable(stream) {
            continue;
        }
        
        for token in stream.split(';') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            
            let namespace = if let Some(colon_pos) = token.find(':') {
                token[..colon_pos].to_string()
            } else {
                "global".to_string()
            };
            
            namespace_streams
                .entry(namespace)
                .or_default()
                .push(token.to_string());
        }
    }
    
    let mut result = Vec::new();
    
    // Add priority namespaces first
    for priority_ns in priorities {
        if let Some(tokens) = namespace_streams.remove(priority_ns) {
            result.extend(tokens);
        }
    }
    
    // Add remaining namespaces in alphabetical order
    let mut remaining: Vec<_> = namespace_streams.into_iter().collect();
    remaining.sort_by(|a, b| a.0.cmp(&b.0));
    
    for (_, tokens) in remaining {
        result.extend(tokens);
    }
    
    result.join("; ")
}

/// Merge and sort tokens by key name
pub fn merge_sorted(streams: &[&str]) -> String {
    let mut all_tokens = Vec::new();
    
    for stream in streams {
        if !is_token_streamable(stream) {
            continue;
        }
        
        for token in stream.split(';') {
            let token = token.trim();
            if !token.is_empty() {
                all_tokens.push(token.to_string());
            }
        }
    }
    
    // Sort by key (part after : or whole token if no :)
    all_tokens.sort_by(|a, b| {
        let key_a = if let Some(eq_pos) = a.find('=') {
            &a[..eq_pos]
        } else {
            a
        };
        let key_b = if let Some(eq_pos) = b.find('=') {
            &b[..eq_pos]
        } else {
            b
        };
        key_a.cmp(key_b)
    });
    
    all_tokens.join("; ")
}

/// Handle collision detection and resolution
pub fn merge_with_collision_policy(
    streams: &[&str], 
    policy: CollisionPolicy
) -> String {
    let mut seen_keys = HashMap::new();
    let mut result_tokens = Vec::new();
    
    for stream in streams {
        if !is_token_streamable(stream) {
            continue;
        }
        
        for token in stream.split(';') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            
            // Extract key from token
            let key = if let Some(eq_pos) = token.find('=') {
                token[..eq_pos].to_string()
            } else {
                token.to_string()
            };
            
            match policy {
                CollisionPolicy::KeepFirst => {
                    if !seen_keys.contains_key(&key) {
                        seen_keys.insert(key, result_tokens.len());
                        result_tokens.push(token.to_string());
                    }
                }
                CollisionPolicy::KeepLast => {
                    if let Some(&pos) = seen_keys.get(&key) {
                        result_tokens[pos] = token.to_string();
                    } else {
                        seen_keys.insert(key, result_tokens.len());
                        result_tokens.push(token.to_string());
                    }
                }
                CollisionPolicy::Annotate => {
                    if seen_keys.contains_key(&key) {
                        // Mark this as a duplicate
                        result_tokens.push(format!("dupe:{}=true; {}", key, token));
                    } else {
                        seen_keys.insert(key, result_tokens.len());
                        result_tokens.push(token.to_string());
                    }
                }
            }
        }
    }
    
    result_tokens.join("; ")
}

/// Macro for clean merge syntax
#[macro_export]
macro_rules! merge {
    // merge!(stream1, stream2, stream3)
    ($($stream:expr),+) => {{
        let streams = vec![$($stream),+];
        $crate::xstream::merge::merge_concat(&streams)
    }};
    
    // merge!(strategy: Interleave, stream1, stream2, stream3)
    (strategy: $strategy:expr, $($stream:expr),+) => {{
        let streams = vec![$($stream),+];
        $crate::xstream::merge::merge_with_strategy(&streams, $strategy)
    }};
    
    // merge!(policy: KeepFirst, stream1, stream2, stream3)
    (policy: $policy:expr, $($stream:expr),+) => {{
        let streams = vec![$($stream),+];
        $crate::xstream::merge::merge_with_collision_policy(&streams, $policy)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_concat() {
        let stream1 = "ui:click=\"btn1\"";
        let stream2 = "db:host=\"localhost\"";
        let stream3 = "log:level=\"info\"";
        
        let result = merge_concat(&[stream1, stream2, stream3]);
        assert_eq!(result, "ui:click=\"btn1\"; db:host=\"localhost\"; log:level=\"info\"");
    }

    #[test]
    fn test_merge_macro() {
        let ui = "ui:click=\"btn1\"";
        let db = "db:host=\"localhost\"";
        let log = "log:level=\"info\"";
        
        let result = merge!(ui, db, log);
        assert!(result.contains("ui:click=\"btn1\""));
        assert!(result.contains("db:host=\"localhost\""));
        assert!(result.contains("log:level=\"info\""));
    }

    #[test]
    fn test_merge_interleave() {
        let stream1 = "a=\"1\"; b=\"2\"";
        let stream2 = "c=\"3\"; d=\"4\"";
        
        let result = merge_interleave(&[stream1, stream2]);
        // Should interleave: a=1, c=3, b=2, d=4
        assert!(result.contains("a=\"1\""));
        assert!(result.contains("c=\"3\""));
        assert!(result.contains("b=\"2\""));
        assert!(result.contains("d=\"4\""));
    }

    #[test]
    fn test_merge_with_strategy() {
        let ui = "ui:theme=\"dark\"";
        let db = "db:host=\"localhost\"";
        
        let result = merge!(strategy: MergeStrategy::Sort, ui, db);
        assert!(result.contains("db:host=\"localhost\""));
        assert!(result.contains("ui:theme=\"dark\""));
    }

    #[test]
    fn test_collision_policy() {
        let stream1 = "key=\"first\"";
        let stream2 = "key=\"second\"";
        
        let keep_first = merge_with_collision_policy(&[stream1, stream2], CollisionPolicy::KeepFirst);
        assert!(keep_first.contains("key=\"first\""));
        assert!(!keep_first.contains("key=\"second\""));
        
        let keep_last = merge_with_collision_policy(&[stream1, stream2], CollisionPolicy::KeepLast);
        assert!(!keep_last.contains("key=\"first\""));
        assert!(keep_last.contains("key=\"second\""));
    }
}