// XStream REAL Gate Operations - Actually using RSB Streamable!
// This is what gate.rs should have been...

use rsb::prelude::*;
use crate::xstream::types::{TokenBucket, BucketMode, is_token_streamable};

/// Gate with visual state tracking for colored display
#[derive(Debug)]
pub struct GateState {
    pub current_stream: usize,
    pub tokens_processed: usize,
    pub switches: Vec<(usize, String)>, // (position, which_stream)
}

/// Gate operation that controls stream flow based on conditions
pub struct Gate;

impl Streamable for Gate {
    type Args = GateCondition;
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        // Parse the input stream properly using TokenBucket
        let bucket = match TokenBucket::from_str(stdin, BucketMode::Hybrid) {
            Ok(b) => b,
            Err(_) => return String::new(), // Invalid input blocks the stream
        };
        
        match args {
            GateCondition::MinTokens(min) => {
                let total_tokens = bucket.data.values().map(|ns_data| ns_data.len()).sum::<usize>();
                if total_tokens >= min {
                    stdin.to_string()
                } else {
                    String::new() // Block the stream
                }
            }
            GateCondition::MaxTokens(max) => {
                let total_tokens = bucket.data.values().map(|ns_data| ns_data.len()).sum::<usize>();
                if total_tokens <= max {
                    stdin.to_string()
                } else {
                    // Truncate to max tokens - reconstruct properly
                    let mut result_tokens = Vec::new();
                    let mut count = 0;
                    
                    for (namespace, data) in &bucket.data {
                        for (key, value) in data {
                            if count >= max { break; }
                            if namespace == "global" {
                                result_tokens.push(format!("{}=\"{}\"", key, value));
                            } else {
                                result_tokens.push(format!("{}:{}=\"{}\"", namespace, key, value));
                            }
                            count += 1;
                        }
                        if count >= max { break; }
                    }
                    result_tokens.join("; ")
                }
            }
            GateCondition::RequireNamespace(ns) => {
                // Only pass if namespace exists in parsed data
                if bucket.data.contains_key(&ns) {
                    stdin.to_string()
                } else {
                    String::new()
                }
            }
            GateCondition::TokenCount(count) => {
                let total_tokens = bucket.data.values().map(|ns_data| ns_data.len()).sum::<usize>();
                if total_tokens == count {
                    stdin.to_string()
                } else {
                    String::new()
                }
            }
            GateCondition::ContainsValue(value) => {
                // Check if any parsed value matches
                let has_value = bucket.data.values()
                    .any(|ns_data| ns_data.values().any(|v| v == &value));
                    
                if has_value {
                    stdin.to_string()
                } else {
                    String::new()
                }
            }
        }
    }
}

// XOR Gate Operations from fake version

/// XOR Gate - alternates between two streams, only one passes at a time
pub struct XorGate;

impl Streamable for XorGate {
    type Args = String; // Second stream
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        xor_gate(stdin, &args)
    }
}

/// Multi-stream XOR gate - cycles through multiple streams
pub struct MultiXorGate;

impl Streamable for MultiXorGate {
    type Args = Vec<String>; // Other streams
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        let mut all_streams = vec![stdin];
        all_streams.extend(args.iter().map(|s| s.as_str()));
        multi_xor_gate(&all_streams)
    }
}

/// Timed gate - switch streams every N tokens  
pub struct TimedGate;

impl Streamable for TimedGate {
    type Args = (usize, Vec<String>); // (switch_every, other_streams)
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        let (switch_every, other_streams) = args;
        let mut all_streams = vec![stdin];
        all_streams.extend(other_streams.iter().map(|s| s.as_str()));
        timed_gate(&all_streams, switch_every)
    }
}

// Core gate functions from fake version (adapted for RSB)

/// XOR Gate - alternates between two streams, only one passes at a time
pub fn xor_gate(stream_a: &str, stream_b: &str) -> String {
    if !is_token_streamable(stream_a) || !is_token_streamable(stream_b) {
        return String::new();
    }
    
    let tokens_a: Vec<&str> = stream_a.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    let tokens_b: Vec<&str> = stream_b.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    
    let mut result = Vec::new();
    let max_len = std::cmp::max(tokens_a.len(), tokens_b.len());
    
    // XOR alternation - switch between streams
    for i in 0..max_len * 2 {  // We want to alternate taking from each stream
        if i % 2 == 0 {
            // Even index - use stream A
            let token_index = i / 2;
            if token_index < tokens_a.len() {
                result.push(tokens_a[token_index]);
            }
        } else {
            // Odd index - use stream B  
            let token_index = i / 2;
            if token_index < tokens_b.len() {
                result.push(tokens_b[token_index]);
            }
        }
    }
    
    result.join("; ")
}

/// Multi-stream XOR gate - cycles through multiple streams
pub fn multi_xor_gate(streams: &[&str]) -> String {
    let valid_streams: Vec<Vec<&str>> = streams
        .iter()
        .filter(|s| is_token_streamable(s))
        .map(|s| s.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect())
        .collect();
    
    if valid_streams.is_empty() {
        return String::new();
    }
    
    let total_tokens: usize = valid_streams.iter().map(|s| s.len()).sum();
    let mut result = Vec::new();
    let mut stream_indices = vec![0; valid_streams.len()];
    
    // Cycle through streams taking one token at a time
    for i in 0..total_tokens {
        let stream_index = i % valid_streams.len();
        if stream_indices[stream_index] < valid_streams[stream_index].len() {
            result.push(valid_streams[stream_index][stream_indices[stream_index]]);
            stream_indices[stream_index] += 1;
        } else {
            // This stream is exhausted, find the next non-exhausted stream
            for j in 1..valid_streams.len() {
                let alt_index = (stream_index + j) % valid_streams.len();
                if stream_indices[alt_index] < valid_streams[alt_index].len() {
                    result.push(valid_streams[alt_index][stream_indices[alt_index]]);
                    stream_indices[alt_index] += 1;
                    break;
                }
            }
        }
    }
    
    result.join("; ")
}

/// Timed gate - switch streams every N tokens
pub fn timed_gate(streams: &[&str], switch_every: usize) -> String {
    let valid_streams: Vec<Vec<&str>> = streams
        .iter()
        .filter(|s| is_token_streamable(s))
        .map(|s| s.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect())
        .collect();
    
    if valid_streams.is_empty() || switch_every == 0 {
        return String::new();
    }
    
    let total_tokens: usize = valid_streams.iter().map(|s| s.len()).sum();
    let mut result = Vec::new();
    let mut current_stream = 0;
    let mut tokens_from_current = 0;
    
    for _token_index in 0..total_tokens {
        // Switch stream if we've taken enough tokens
        if tokens_from_current >= switch_every {
            current_stream = (current_stream + 1) % valid_streams.len();
            tokens_from_current = 0;
        }
        
        // Find the actual token from the current stream
        let stream_token_index = tokens_from_current;
        if stream_token_index < valid_streams[current_stream].len() {
            result.push(valid_streams[current_stream][stream_token_index]);
            tokens_from_current += 1;
        }
    }
    
    result.join("; ")
}

/// XOR gate with state tracking for visualization
pub fn xor_gate_with_state(stream_a: &str, stream_b: &str) -> (String, GateState) {
    let mut state = GateState {
        current_stream: 0,
        tokens_processed: 0,
        switches: Vec::new(),
    };
    
    if !is_token_streamable(stream_a) || !is_token_streamable(stream_b) {
        return (String::new(), state);
    }
    
    let tokens_a: Vec<&str> = stream_a.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    let tokens_b: Vec<&str> = stream_b.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    
    let mut result = Vec::new();
    let max_len = std::cmp::max(tokens_a.len(), tokens_b.len());
    
    for i in 0..max_len * 2 {  // Match the fixed XOR logic
        if i % 2 == 0 {
            // Even index - use stream A
            let token_index = i / 2;
            if token_index < tokens_a.len() {
                result.push(tokens_a[token_index]);
                state.switches.push((result.len() - 1, "A".to_string()));
                state.current_stream = 0;
            }
        } else {
            // Odd index - use stream B  
            let token_index = i / 2;
            if token_index < tokens_b.len() {
                result.push(tokens_b[token_index]);
                state.switches.push((result.len() - 1, "B".to_string()));
                state.current_stream = 1;
            }
        }
        
        state.tokens_processed += 1;
    }
    
    (result.join("; "), state)
}

/// Sync gate - waits until both streams have values before proceeding
pub fn sync_gate(stream_a: &str, stream_b: &str) -> String {
    if !is_token_streamable(stream_a) || !is_token_streamable(stream_b) {
        return String::new();
    }
    
    let tokens_a: Vec<&str> = stream_a.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    let tokens_b: Vec<&str> = stream_b.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    
    // Only proceed with the minimum number of tokens available in both streams
    let sync_length = std::cmp::min(tokens_a.len(), tokens_b.len());
    let mut result = Vec::new();
    
    // Alternate tokens but only up to the sync point
    for i in 0..sync_length * 2 {
        if i % 2 == 0 && i / 2 < tokens_a.len() {
            result.push(tokens_a[i / 2]);
        } else if i % 2 == 1 && i / 2 < tokens_b.len() {
            result.push(tokens_b[i / 2]);
        }
    }
    
    result.join("; ")
}

/// Balance gate - ensures equal token distribution across streams  
pub fn balance_gate(streams: &[&str]) -> String {
    let valid_streams: Vec<Vec<&str>> = streams
        .iter()
        .filter(|s| is_token_streamable(s))
        .map(|s| s.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect())
        .collect();
    
    if valid_streams.is_empty() {
        return String::new();
    }
    
    // Find minimum stream length for balanced distribution
    let min_length = valid_streams.iter().map(|s| s.len()).min().unwrap_or(0);
    let mut result = Vec::new();
    
    // Take exactly min_length tokens from each stream, cycling through them
    for token_index in 0..min_length {
        for stream in &valid_streams {
            if token_index < stream.len() {
                result.push(stream[token_index]);
            }
        }
    }
    
    result.join("; ")
}

/// Wait gate - holds until minimum tokens available in all streams
pub fn wait_gate(streams: &[&str], min_tokens: usize) -> String {
    let valid_streams: Vec<Vec<&str>> = streams
        .iter()
        .filter(|s| is_token_streamable(s))
        .map(|s| s.split(';').map(|t| t.trim()).filter(|t| !t.is_empty()).collect())
        .collect();
    
    if valid_streams.is_empty() {
        return String::new();
    }
    
    // Check if all streams have minimum required tokens
    let all_have_minimum = valid_streams.iter().all(|s| s.len() >= min_tokens);
    
    if !all_have_minimum {
        // Not ready - return empty (in real implementation, this would block)
        return String::new();
    }
    
    // Proceed with cycling through streams, taking all tokens
    let total_tokens: usize = valid_streams.iter().map(|s| s.len()).sum();
    let mut result = Vec::new();
    let mut stream_indices = vec![0; valid_streams.len()];
    
    for i in 0..total_tokens {
        let stream_index = i % valid_streams.len();
        if stream_indices[stream_index] < valid_streams[stream_index].len() {
            result.push(valid_streams[stream_index][stream_indices[stream_index]]);
            stream_indices[stream_index] += 1;
        } else {
            // Find next non-exhausted stream
            for j in 1..valid_streams.len() {
                let alt_index = (stream_index + j) % valid_streams.len();
                if stream_indices[alt_index] < valid_streams[alt_index].len() {
                    result.push(valid_streams[alt_index][stream_indices[alt_index]]);
                    stream_indices[alt_index] += 1;
                    break;
                }
            }
        }
    }
    
    result.join("; ")
}

// Comprehensive Macro System from fake version

/// Macro for clean XOR gate syntax
#[macro_export]
macro_rules! xor {
    // xor!(stream_a, stream_b) -> alternating_stream
    ($stream_a:expr, $stream_b:expr) => {{
        $crate::xstream::real_gate::xor_gate($stream_a, $stream_b)
    }};
}

/// Macro for multi-stream XOR
#[macro_export]
macro_rules! multi_xor {
    ($($stream:expr),+) => {{
        let streams = vec![$($stream),+];
        $crate::xstream::real_gate::multi_xor_gate(&streams)
    }};
}

/// Macro for timed gate
#[macro_export]
macro_rules! timed_gate {
    ($switch_every:expr, $($stream:expr),+) => {{
        let streams = vec![$($stream),+];
        $crate::xstream::real_gate::timed_gate(&streams, $switch_every)
    }};
}

/// Macros for synchronization gates
#[macro_export]
macro_rules! sync_gate {
    ($stream_a:expr, $stream_b:expr) => {{
        $crate::xstream::real_gate::sync_gate($stream_a, $stream_b)
    }};
}

#[macro_export]
macro_rules! balance_gate {
    ($($stream:expr),+) => {{
        let streams = vec![$($stream),+];
        $crate::xstream::real_gate::balance_gate(&streams)
    }};
}

#[macro_export]
macro_rules! wait_gate {
    ($min_tokens:expr, $($stream:expr),+) => {{
        let streams = vec![$($stream),+];
        $crate::xstream::real_gate::wait_gate(&streams, $min_tokens)
    }};
}

/// Sync gate - waits until both streams have minimum tokens
pub struct SyncGate;

impl Streamable for SyncGate {
    type Args = (String, usize); // (other_stream, min_tokens)
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        let (other_stream, min_tokens) = args;
        
        // Parse both streams properly
        let bucket1 = match TokenBucket::from_str(stdin, BucketMode::Hybrid) {
            Ok(b) => b,
            Err(_) => return String::new(),
        };
        
        let bucket2 = match TokenBucket::from_str(&other_stream, BucketMode::Hybrid) {
            Ok(b) => b,
            Err(_) => return String::new(),
        };
        
        let tokens1_count = bucket1.data.values().map(|ns_data| ns_data.len()).sum::<usize>();
        let tokens2_count = bucket2.data.values().map(|ns_data| ns_data.len()).sum::<usize>();
        
        if tokens1_count >= min_tokens && tokens2_count >= min_tokens {
            // Interleave tokens from both streams - reconstruct properly
            let mut result_tokens = Vec::new();
            
            // Convert buckets to token lists
            let mut tokens1 = Vec::new();
            for (namespace, data) in &bucket1.data {
                for (key, value) in data {
                    if namespace == "global" {
                        tokens1.push(format!("{}=\"{}\"", key, value));
                    } else {
                        tokens1.push(format!("{}:{}=\"{}\"", namespace, key, value));
                    }
                }
            }
            
            let mut tokens2 = Vec::new();
            for (namespace, data) in &bucket2.data {
                for (key, value) in data {
                    if namespace == "global" {
                        tokens2.push(format!("{}=\"{}\"", key, value));
                    } else {
                        tokens2.push(format!("{}:{}=\"{}\"", namespace, key, value));
                    }
                }
            }
            
            let min_len = tokens1.len().min(tokens2.len());
            for i in 0..min_len {
                result_tokens.push(tokens1[i].clone());
                result_tokens.push(tokens2[i].clone());
            }
            
            result_tokens.join("; ")
        } else {
            String::new() // Block until both have enough tokens
        }
    }
}

/// Balance gate - ensures equal token distribution
pub struct BalanceGate;

impl Streamable for BalanceGate {
    type Args = Vec<String>; // Multiple streams to balance
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        let mut all_streams = vec![stdin.to_string()];
        all_streams.extend(args);
        
        // Parse all streams using TokenBucket
        let mut parsed_buckets = Vec::new();
        for stream in &all_streams {
            match TokenBucket::from_str(stream, BucketMode::Hybrid) {
                Ok(bucket) => parsed_buckets.push(bucket),
                Err(_) => return String::new(), // Any invalid stream fails the whole gate
            }
        }
        
        if parsed_buckets.is_empty() {
            return String::new();
        }
        
        // Find minimum token count across all streams
        let stream_lengths: Vec<usize> = parsed_buckets
            .iter()
            .map(|bucket| bucket.data.values().map(|ns_data| ns_data.len()).sum::<usize>())
            .collect();
            
        let min_len = stream_lengths.iter().min().copied().unwrap_or(0);
        
        if min_len == 0 {
            return String::new();
        }
        
        // Take equal number of tokens from each stream
        let mut result_tokens = Vec::new();
        for bucket in &parsed_buckets {
            let mut count = 0;
            for (namespace, data) in &bucket.data {
                for (key, value) in data {
                    if count >= min_len { break; }
                    if namespace == "global" {
                        result_tokens.push(format!("{}=\"{}\"", key, value));
                    } else {
                        result_tokens.push(format!("{}:{}=\"{}\"", namespace, key, value));
                    }
                    count += 1;
                }
                if count >= min_len { break; }
            }
        }
        
        result_tokens.join("; ")
    }
}

/// Wait gate - blocks until minimum requirement met across all streams
pub struct WaitGate;

impl Streamable for WaitGate {
    type Args = (usize, Vec<String>); // (min_tokens, other_streams)
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        let (min_tokens, other_streams) = args;
        
        let mut all_streams = vec![stdin.to_string()];
        all_streams.extend(other_streams);
        
        // Check if all streams have minimum tokens using proper parsing
        for stream in &all_streams {
            let bucket = match TokenBucket::from_str(stream, BucketMode::Hybrid) {
                Ok(b) => b,
                Err(_) => return String::new(), // Invalid stream blocks everything
            };
            
            let token_count = bucket.data.values().map(|ns_data| ns_data.len()).sum::<usize>();
            if token_count < min_tokens {
                return String::new(); // Block - not all streams have enough tokens
            }
        }
        
        // All streams pass - return first stream (or could combine them)
        stdin.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum GateCondition {
    MinTokens(usize),         // Minimum tokens required
    MaxTokens(usize),         // Maximum tokens allowed  
    RequireNamespace(String), // Must have this namespace
    TokenCount(usize),        // Exact token count
    ContainsValue(String),    // Must contain this value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_min_tokens() {
        let input = "a:x=\"1\"; b:y=\"2\"; c:z=\"3\"".to_string();
        
        // Should pass with min 2
        let result = input.clone().stream_apply(Gate, GateCondition::MinTokens(2));
        assert_eq!(result, input);
        
        // Should block with min 5
        let blocked = input.stream_apply(Gate, GateCondition::MinTokens(5));
        assert_eq!(blocked, "");
    }
    
    #[test]
    fn test_gate_max_tokens() {
        let input = "a:x=\"1\"; b:y=\"2\"; c:z=\"3\"; d:w=\"4\"; e:v=\"5\"".to_string();
        
        // Should truncate to 3 tokens
        let result = input.stream_apply(Gate, GateCondition::MaxTokens(3));
        assert_eq!(result, "a:x=\"1\"; b:y=\"2\"; c:z=\"3\"");
    }
    
    #[test]
    fn test_gate_require_namespace() {
        let input = "ui:click=\"btn\"; db:host=\"local\"; api:status=\"ok\"".to_string();
        
        // Should pass - has ui namespace
        let result = input.clone().stream_apply(Gate, GateCondition::RequireNamespace("ui".to_string()));
        assert_eq!(result, input);
        
        // Should block - no auth namespace
        let blocked = input.stream_apply(Gate, GateCondition::RequireNamespace("auth".to_string()));
        assert_eq!(blocked, "");
    }
    
    #[test]
    fn test_sync_gate() {
        let stream1 = "a:x=\"1\"; a:y=\"2\"; a:z=\"3\"".to_string();
        let stream2 = "b:x=\"4\"; b:y=\"5\"".to_string();
        
        let result = stream1.stream_apply(SyncGate, (stream2, 2));
        
        // Should interleave: a:x="1", b:x="4", a:y="2", b:y="5"
        assert!(result.contains("a:x=\"1\""));
        assert!(result.contains("b:x=\"4\""));
        assert!(result.contains("a:y=\"2\""));
        assert!(result.contains("b:y=\"5\""));
    }
}