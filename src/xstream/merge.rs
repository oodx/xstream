// XStream REAL Merge Operations - Complete RSB Streamable Implementation
// Consolidating all features from fake merge.rs with proper RSB patterns

use rsb::prelude::*;
use std::collections::{HashSet, HashMap};
use crate::xstream::types::is_token_streamable;

/// Merge operation that combines multiple streams
pub struct Merge;

impl Streamable for Merge {
    type Args = MergeStrategy;
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        // Split input by newlines (each line is a forked stream)
        let streams: Vec<&str> = stdin.lines().collect();
        
        merge_with_strategy(&streams, args)
    }
}

// Core merge functions from fake version (now using RSB compatible format)

/// Basic merge - concatenate streams in order
pub fn merge_concat(streams: &[&str]) -> String {
    let mut all_tokens = Vec::new();
    
    for stream in streams {
        if stream.trim().is_empty() {
            continue;
        }
        
        // Handle RSB format "namespace: tokens" or plain tokens
        let tokens_str = if let Some((_, tokens)) = stream.split_once(": ") {
            tokens
        } else if is_token_streamable(stream) {
            stream
        } else {
            continue; // Skip invalid streams
        };
        
        // Split tokens and add to collection
        for token in tokens_str.split(';') {
            let token = token.trim();
            if !token.is_empty() {
                all_tokens.push(token);
            }
        }
    }
    
    if all_tokens.is_empty() {
        return String::new();
    }
    
    all_tokens.join("; ")
}

/// Merge with strategy
pub fn merge_with_strategy(streams: &[&str], strategy: MergeStrategy) -> String {
    match strategy {
        MergeStrategy::Concat => {
            // Extract tokens from RSB format
            let mut all_tokens = Vec::new();
            for stream in streams {
                if let Some((_, tokens)) = stream.split_once(": ") {
                    all_tokens.extend(tokens.split("; "));
                } else {
                    // Handle streams without namespace prefix
                    all_tokens.extend(stream.split("; "));
                }
            }
            all_tokens.join("; ")
        }
        MergeStrategy::Interleave => merge_interleave(streams),
        MergeStrategy::Priority(priorities) => merge_priority(streams, &priorities),
        MergeStrategy::Sort => merge_sorted(streams),
        MergeStrategy::Dedupe => {
            // Remove duplicate tokens
            let mut seen = HashSet::new();
            let mut result = Vec::new();
            
            for stream in streams {
                let tokens = if let Some((_, tokens)) = stream.split_once(": ") {
                    tokens
                } else {
                    stream
                };
                
                for token in tokens.split("; ") {
                    let token = token.trim();
                    if !token.is_empty() && seen.insert(token.to_string()) {
                        result.push(token);
                    }
                }
            }
            result.join("; ")
        }
        MergeStrategy::ByNamespace => {
            // Group by namespace and merge within each group
            let mut namespace_groups: HashMap<String, Vec<&str>> = HashMap::new();
            
            for stream in streams {
                if let Some((namespace, tokens)) = stream.split_once(": ") {
                    let entry = namespace_groups.entry(namespace.to_string()).or_insert_with(Vec::new);
                    entry.extend(tokens.split("; "));
                }
            }
            
            let mut result = Vec::new();
            for (namespace, tokens) in namespace_groups {
                result.push(format!("{}: {}", namespace, tokens.join("; ")));
            }
            result.join("\n")
        }
    }
}

/// Interleave tokens from multiple streams in round-robin
pub fn merge_interleave(streams: &[&str]) -> String {
    let token_lists: Vec<Vec<&str>> = streams
        .iter()
        .filter_map(|s| {
            // Handle RSB format "namespace: tokens" or plain tokens
            let tokens_str = if let Some((_, tokens)) = s.split_once(": ") {
                tokens
            } else if is_token_streamable(s) {
                s
            } else {
                return None; // Skip invalid streams
            };
            
            // Only create token list if the tokens part is valid
            if is_token_streamable(tokens_str) {
                Some(tokens_str.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect())
            } else {
                None
            }
        })
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
        
        let tokens_str = if let Some((_, tokens)) = stream.split_once(": ") {
            tokens
        } else {
            stream
        };
        
        for token in tokens_str.split(';') {
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
        if stream.trim().is_empty() {
            continue;
        }
        
        // Handle RSB format "namespace: tokens" or plain tokens
        let tokens_str = if let Some((_, tokens)) = stream.split_once(": ") {
            tokens
        } else if is_token_streamable(stream) {
            stream
        } else {
            continue; // Skip invalid streams
        };
        
        for token in tokens_str.split(';') {
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
        
        let tokens_str = if let Some((_, tokens)) = stream.split_once(": ") {
            tokens
        } else {
            stream
        };
        
        for token in tokens_str.split(';') {
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

/// Macro for clean merge syntax - RSB compatible
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

/// Selective merge - only merge streams matching criteria
pub struct SelectiveMerge;

impl Streamable for SelectiveMerge {
    type Args = (MergeStrategy, MergeFilter);
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        let (strategy, filter) = args;
        let streams: Vec<&str> = stdin.lines().collect();
        
        // Filter streams first
        let filtered_streams: Vec<&str> = streams
            .into_iter()
            .filter(|stream| {
                match &filter {
                    MergeFilter::NamespaceOnly(ns) => stream.starts_with(&format!("{}:", ns)),
                    MergeFilter::ContainsValue(val) => stream.contains(val),
                    MergeFilter::MinTokens(min) => {
                        let token_count = if let Some((_, tokens)) = stream.split_once(": ") {
                            tokens.split("; ").count()
                        } else {
                            stream.split("; ").count()
                        };
                        token_count >= *min
                    }
                }
            })
            .collect();
        
        // Apply merge strategy to filtered streams
        let filtered_input = filtered_streams.join("\n");
        filtered_input.stream_apply(Merge, strategy)
    }
}

/// Weighted merge - merge with priority/weighting
pub struct WeightedMerge;

impl Streamable for WeightedMerge {
    type Args = Vec<(String, f32)>; // (namespace, weight)
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        let streams: Vec<&str> = stdin.lines().collect();
        let mut weighted_tokens: Vec<&str> = Vec::new();
        
        for stream in streams {
            if let Some((namespace, tokens)) = stream.split_once(": ") {
                let weight = args.iter()
                    .find(|(ns, _)| ns == namespace)
                    .map(|(_, w)| *w)
                    .unwrap_or(1.0);
                
                let token_list: Vec<&str> = tokens.split("; ").collect();
                let take_count = (token_list.len() as f32 * weight).round() as usize;
                
                weighted_tokens.extend(&token_list[..take_count.min(token_list.len())]);
            }
        }
        
        weighted_tokens.join("; ")
    }
}

#[derive(Debug, Clone)]
pub enum MergeStrategy {
    Concat,       // Concatenate all streams
    Interleave,   // Interleave tokens from each stream
    Dedupe,       // Remove duplicates
    ByNamespace,  // Group by namespace
    Priority(Vec<String>), // Priority order - channels listed first get precedence
    Sort,         // Sort tokens by key name
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

#[derive(Debug, Clone)]
pub enum MergeFilter {
    NamespaceOnly(String),  // Only merge specific namespace
    ContainsValue(String),  // Only merge streams containing value
    MinTokens(usize),       // Only merge streams with min tokens
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
    
    #[test]
    fn test_merge_priority() {
        let stream1 = "ui:a=\"1\"; db:b=\"2\"";
        let stream2 = "log:c=\"3\"; ui:d=\"4\"";
        
        let result = merge_priority(&[stream1, stream2], &["ui".to_string(), "log".to_string()]);
        
        // ui tokens should come first, then log, then db
        let pos_ui = result.find("ui:").unwrap_or(usize::MAX);
        let pos_log = result.find("log:").unwrap_or(usize::MAX);
        let pos_db = result.find("db:").unwrap_or(usize::MAX);
        
        assert!(pos_ui < pos_log);
        assert!(pos_log < pos_db);
    }
    
    #[test]
    fn test_rsb_streamable_merge() {
        let input = "ui: ui:click=\"btn1\"\ndb: db:host=\"localhost\"";
        let result = input.to_string().stream_apply(Merge, MergeStrategy::Concat);
        
        assert!(result.contains("ui:click=\"btn1\""));
        assert!(result.contains("db:host=\"localhost\""));
    }
    
    #[test]
    fn test_merge_dedupe_streamable() {
        let input = "ui: ui:click=\"btn1\"\ndb: ui:click=\"btn1\""; // Duplicate token
        let result = input.to_string().stream_apply(Merge, MergeStrategy::Dedupe);
        
        // Should only have one instance of ui:click="btn1"
        let count = result.matches("ui:click=\"btn1\"").count();
        assert_eq!(count, 1);
    }
    
    #[test]
    fn test_merge_interleave_streamable() {
        let input = "ui: ui:a=\"1\"; ui:b=\"2\"\ndb: db:x=\"9\"; db:y=\"8\"";
        let result = input.to_string().stream_apply(Merge, MergeStrategy::Interleave);
        
        // Should interleave: ui:a="1", db:x="9", ui:b="2", db:y="8"
        assert!(result.contains("ui:a=\"1\""));
        assert!(result.contains("db:x=\"9\""));
    }
}