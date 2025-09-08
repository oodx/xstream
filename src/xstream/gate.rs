// XStream Gate Operations - Stream switching and XOR logic
// Control flow gates that alternate between input channels

use crate::xstream::types::is_token_streamable;

/// Gate modes for different switching strategies
#[derive(Debug, Clone)]
pub enum GateMode {
    /// XOR - only one stream passes at a time, alternating
    Xor,
    /// Timed - switch every N tokens
    Timed(usize),
    /// Priority - switch when priority channel has data
    Priority(String),
    /// Random - randomly pick which stream passes
    Random,
}

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
    for i in 0..max_len {
        if i % 2 == 0 {
            // Even index - use stream A
            if i < tokens_a.len() {
                result.push(tokens_a[i]);
            }
        } else {
            // Odd index - use stream B  
            if i < tokens_b.len() {
                result.push(tokens_b[i]);
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
    
    let max_len = valid_streams.iter().map(|s| s.len()).max().unwrap_or(0);
    let mut result = Vec::new();
    
    // Cycle through streams in XOR fashion
    for i in 0..max_len {
        let stream_index = i % valid_streams.len();
        if i / valid_streams.len() < valid_streams[stream_index].len() {
            result.push(valid_streams[stream_index][i / valid_streams.len()]);
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

/// Gate with visual state tracking for colored display
pub struct GateState {
    pub current_stream: usize,
    pub tokens_processed: usize,
    pub switches: Vec<(usize, String)>, // (position, which_stream)
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
    
    for i in 0..max_len {
        let use_stream_a = i % 2 == 0;
        
        if use_stream_a && i < tokens_a.len() {
            result.push(tokens_a[i]);
            state.switches.push((result.len() - 1, "A".to_string()));
            state.current_stream = 0;
        } else if !use_stream_a && i < tokens_b.len() {
            result.push(tokens_b[i]);
            state.switches.push((result.len() - 1, "B".to_string()));
            state.current_stream = 1;
        }
        
        state.tokens_processed += 1;
    }
    
    (result.join("; "), state)
}

/// Macro for clean gate syntax
#[macro_export]
macro_rules! xor {
    // xor!(stream_a, stream_b) -> alternating_stream
    ($stream_a:expr, $stream_b:expr) => {{
        $crate::xstream::gate::xor_gate($stream_a, $stream_b)
    }};
}

/// Macro for multi-stream XOR
#[macro_export]
macro_rules! multi_xor {
    ($($stream:expr),+) => {{
        let streams = vec![$($stream),+];
        $crate::xstream::gate::multi_xor_gate(&streams)
    }};
}

/// Macro for timed gate
#[macro_export]
macro_rules! timed_gate {
    ($switch_every:expr, $($stream:expr),+) => {{
        let streams = vec![$($stream),+];
        $crate::xstream::gate::timed_gate(&streams, $switch_every)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_gate() {
        let stream_a = "ui:theme=\"dark\"; ui:lang=\"en\"";
        let stream_b = "db:host=\"localhost\"; db:port=\"5432\"";
        
        let result = xor_gate(stream_a, stream_b);
        let tokens: Vec<&str> = result.split(';').map(|t| t.trim()).collect();
        
        // Should alternate: ui:theme, db:host, ui:lang, db:port
        assert_eq!(tokens.len(), 4);
        assert!(tokens[0].contains("ui:theme"));
        assert!(tokens[1].contains("db:host"));
        assert!(tokens[2].contains("ui:lang"));
        assert!(tokens[3].contains("db:port"));
    }

    #[test]
    fn test_xor_macro() {
        let ui = "ui:click=\"btn1\"";
        let db = "db:query=\"select\"";
        
        let result = xor!(ui, db);
        assert!(result.contains("ui:click"));
        assert!(result.contains("db:query"));
    }

    #[test]
    fn test_multi_xor_gate() {
        let stream1 = "a=\"1\"";
        let stream2 = "b=\"2\"";
        let stream3 = "c=\"3\"";
        
        let result = multi_xor_gate(&[stream1, stream2, stream3]);
        // Should cycle through streams
        assert!(result.contains("a=\"1\""));
        assert!(result.contains("b=\"2\""));
        assert!(result.contains("c=\"3\""));
    }

    #[test]
    fn test_timed_gate() {
        let stream1 = "a=\"1\"; a=\"2\"; a=\"3\"";
        let stream2 = "b=\"1\"; b=\"2\"; b=\"3\"";
        
        let result = timed_gate(&[stream1, stream2], 2);
        // Should take 2 from first stream, then 2 from second, etc.
        assert!(result.contains("a=\"1\""));
        assert!(result.contains("b=\"1\""));
    }

    #[test]
    fn test_xor_gate_with_state() {
        let stream_a = "ui:theme=\"dark\"";
        let stream_b = "db:host=\"localhost\"";
        
        let (result, state) = xor_gate_with_state(stream_a, stream_b);
        
        assert!(!result.is_empty());
        assert_eq!(state.switches.len(), 2);
        assert_eq!(state.switches[0].1, "A");
        assert_eq!(state.switches[1].1, "B");
    }
}