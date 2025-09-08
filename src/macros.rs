//! XStream User-Friendly Macros
//! 
//! These macros make complex xstream operations simple for end users.
//! They focus on readability and ease of use.
//! 
//! Note: These macros are designed to NOT collide with RSB macros.
//! They use distinct naming patterns and focus on xstream-specific operations.

// Imports for macro internal use only

/// Create a stream processor with fluent API
/// 
/// Example: `xstream!(input).fork(["ui", "db"]).merge(MergeStrategy::Concat)`
/// 
/// This creates a fluent interface for chaining xstream operations.
#[macro_export]
macro_rules! xstream {
    ($input:expr) => {{
        $crate::adapter::XStreamAdapter::new().process_stream($input)
    }};
}

/// Quick fork operation with automatic color assignment
/// 
/// Example: `fork_colored!(input, "ui", "db", "api")`
/// 
/// This forks the input stream by the specified namespaces and applies
/// automatic color coding for visualization.
#[macro_export] 
macro_rules! fork_colored {
    ($input:expr, $($ns:expr),*) => {{
        let namespaces = vec![$($ns),*];
        let mut adapter = $crate::adapter::XStreamAdapter::new();
        adapter.split_and_process($input, &namespaces)
    }};
}

/// Pipeline macro for complex operations
/// 
/// Example: `pipeline!(input => fork(["ui", "db"]) => gate(min_tokens: 2) => merge(Interleave))`
/// 
/// This creates a processing pipeline with multiple steps.
#[macro_export]
macro_rules! pipeline {
    // Base case: just input
    ($input:expr) => { $input.to_string() };
    
    // Fork operation
    ($input:expr => fork([$($ns:expr),*]) $( => $rest:tt)*) => {{
        use rsb::prelude::*;
        let forked = $input.to_string().stream_apply(
            $crate::xstream::Fork, 
            vec![$($ns.to_string()),*]
        );
        pipeline!(forked $( => $rest)*)
    }};
    
    // Fork operation with variable
    ($input:expr => fork($ns_var:expr) $( => $rest:tt)*) => {{
        use rsb::prelude::*;
        let forked = $input.to_string().stream_apply(
            $crate::xstream::Fork, 
            $ns_var.iter().map(|s| s.to_string()).collect::<Vec<_>>()
        );
        pipeline!(forked $( => $rest)*)
    }};
    
    // Gate operation with minimum tokens
    ($input:expr => gate(min_tokens: $min:expr) $( => $rest:tt)*) => {{
        let lines: Vec<&str> = $input.lines().collect();
        let filtered: Vec<&str> = lines.into_iter()
            .filter(|line| {
                if let Some((_, tokens)) = line.split_once(": ") {
                    tokens.split(';').filter(|t| !t.trim().is_empty()).count() >= $min
                } else {
                    $input.split(';').filter(|t| !t.trim().is_empty()).count() >= $min
                }
            })
            .collect();
        let gated = filtered.join("\n");
        pipeline!(gated $( => $rest)*)
    }};
    
    // Merge operation
    ($input:expr => merge($strategy:tt) $( => $rest:tt)*) => {{
        use rsb::prelude::*;
        let merged = $input.to_string().stream_apply($crate::xstream::Merge, $strategy);
        pipeline!(merged $( => $rest)*)
    }};
    
    // Transform operation (for future extensibility)
    ($input:expr => transform($op:expr) $( => $rest:tt)*) => {{
        // For now, just pass through - can be extended later
        let transformed = $input.to_string();
        pipeline!(transformed $( => $rest)*)
    }};
}

/// Easy stream generation for testing
/// 
/// Example: `generate_stream!(namespaces: ["ui", "db"], tokens_per_ns: 3, colors: true)`
/// 
/// This generates test streams with the specified namespaces and token counts.
#[macro_export]
macro_rules! generate_stream {
    // With colors enabled
    (namespaces: [$($ns:expr),*], tokens_per_ns: $count:expr, colors: true) => {{
        let namespaces = vec![$($ns),*];
        $crate::adapter::XStreamAdapter::test_streams(&namespaces, $count)
    }};
    
    // Without colors
    (namespaces: [$($ns:expr),*], tokens_per_ns: $count:expr, colors: false) => {{
        let namespaces = vec![$($ns),*];
        let mut streams = Vec::new();
        for (idx, ns) in namespaces.iter().enumerate() {
            let tokens: Vec<String> = (0..$count)
                .map(|i| format!("{}:key{}=\"value{}\"", ns, i, idx * $count + i))
                .collect();
            streams.push(tokens.join("; "));
        }
        streams
    }};
    
    // Default without colors
    (namespaces: [$($ns:expr),*], tokens_per_ns: $count:expr) => {{
        generate_stream!(namespaces: [$($ns),*], tokens_per_ns: $count, colors: false)
    }};
}

/// Testing macro for stream operations
/// 
/// Example: `test_stream!(input, expected, |s| s.fork(["ui"]).merge(MergeStrategy::Concat))`
/// 
/// This provides a convenient way to test stream operations with expected results.
#[macro_export]
macro_rules! test_stream {
    ($input:expr, $expected:expr, |$s:ident| $ops:expr) => {{
        let mut adapter = $crate::adapter::XStreamAdapter::new_no_color();
        let mut $s = adapter.process_stream($input);
        let result = $ops.collect();
        
        // Return a test result structure
        TestResult {
            input: $input.to_string(),
            expected: $expected.to_string(),
            actual: result.clone(),
            passed: result.contains(&$expected) || $expected.is_empty(),
        }
    }};
}

/// Builder macro for creating processing pipelines
/// 
/// Example: `build_pipeline!(fork => ["ui", "db"]; gate => 2; merge => Concat)`
/// 
/// This creates reusable pipeline configurations.
#[macro_export]
macro_rules! build_pipeline {
    // Fork step
    (fork => [$($ns:expr),*] $(; $($rest:tt)*)*) => {{
        let builder = $crate::adapter::XStreamAdapter::pipeline()
            .fork(&[$($ns),*]);
        build_pipeline!(builder $(; $($rest)*)*)
    }};
    
    // Gate step (continuing from builder)
    ($builder:ident ; gate => $min:expr $(; $($rest:tt)*)*) => {{
        let builder = $builder.gate($min);
        build_pipeline!(builder $(; $($rest)*)*)
    }};
    
    // Merge step (continuing from builder)
    ($builder:ident ; merge => $strategy:tt $(; $($rest:tt)*)*) => {{
        let builder = $builder.merge($strategy);
        build_pipeline!(builder $(; $($rest)*)*)
    }};
    
    // Filter step (continuing from builder)
    ($builder:ident ; filter => $condition:expr $(; $($rest:tt)*)*) => {{
        let builder = $builder.filter($condition);
        build_pipeline!(builder $(; $($rest)*)*)
    }};
    
    // Base case - return the final builder
    ($builder:ident) => { $builder };
}

/// Quick validation macro
/// 
/// Example: `validate_stream!(stream => "Expected error message")`
/// 
/// This validates that a stream conforms to the expected format.
#[macro_export]
macro_rules! validate_stream {
    ($stream:expr) => {{
        $crate::adapter::XStreamAdapter::validate_stream($stream)
    }};
    
    ($stream:expr => $error_msg:expr) => {{
        match $crate::adapter::XStreamAdapter::validate_stream($stream) {
            Ok(()) => Ok(()),
            Err(_) => Err($crate::adapter::AdapterError::InvalidInput($error_msg.to_string())),
        }
    }};
}

/// Configuration macro for creating common stream setups
/// 
/// Example: `setup_streams!(config: ui="theme=dark", db="host=localhost")`
/// 
/// This creates pre-configured streams for common patterns.
#[macro_export]
macro_rules! setup_streams {
    (config: $($ns:ident = $tokens:expr),*) => {{
        let mut streams = std::collections::HashMap::new();
        $(
            streams.insert(stringify!($ns).to_string(), $tokens.to_string());
        )*
        streams
    }};
}

/// Batch processing macro for multiple streams
/// 
/// Example: `batch_process!(streams: [stream1, stream2] => fork(["ui"]))`
/// 
/// This processes multiple streams with the same operation.
#[macro_export]
macro_rules! batch_process {
    (streams: [$($stream:expr),*] => fork([$($ns:expr),*])) => {{
        let namespaces = vec![$($ns),*];
        let mut results = Vec::new();
        $(
            let result = pipeline!($stream => fork(namespaces.clone()));
            results.push(result);
        )*
        results
    }};
}

/// Debug macro for stream visualization
/// 
/// Example: `debug_stream!(stream, "Processing UI tokens")`
/// 
/// This provides colored debug output for stream contents.
#[macro_export]
macro_rules! debug_stream {
    ($stream:expr) => {{
        eprintln!("{}[DEBUG]{} Stream: {}", 
                 $crate::colors::get_color("cyan"), 
                 $crate::colors::RESET, 
                 $stream);
    }};
    
    ($stream:expr, $msg:expr) => {{
        eprintln!("{}[DEBUG]{} {}: {}", 
                 $crate::colors::get_color("cyan"), 
                 $crate::colors::RESET, 
                 $msg,
                 $stream);
    }};
}

/// Conditional processing macro
/// 
/// Example: `process_if!(stream, has_namespace("ui") => fork(["ui", "ux"]))`
/// 
/// This allows conditional processing based on stream content.
#[macro_export]
macro_rules! process_if {
    ($stream:expr, has_namespace($ns:expr) => $($ops:tt)*) => {{
        if $stream.contains(&format!("{}:", $ns)) {
            pipeline!($stream => $($ops)*)
        } else {
            $stream.to_string()
        }
    }};
    
    ($stream:expr, contains($pattern:expr) => $($ops:tt)*) => {{
        if $stream.contains($pattern) {
            pipeline!($stream => $($ops)*)
        } else {
            $stream.to_string()
        }
    }};
    
    ($stream:expr, has_min_tokens($min:expr) => $($ops:tt)*) => {{
        let token_count = $stream.split(';').filter(|t| !t.trim().is_empty()).count();
        if token_count >= $min {
            pipeline!($stream => $($ops)*)
        } else {
            $stream.to_string()
        }
    }};
}

/// Helper struct for test results
pub struct TestResult {
    pub input: String,
    pub expected: String,
    pub actual: String,
    pub passed: bool,
}

impl TestResult {
    pub fn assert_passed(&self) {
        if !self.passed {
            panic!("Test failed!\nInput: {}\nExpected: {}\nActual: {}", 
                   self.input, self.expected, self.actual);
        }
    }
    
    pub fn print_result(&self) {
        use crate::colors::{get_color, RESET};
        
        let status_color = if self.passed { "green" } else { "red" };
        let status = if self.passed { "PASS" } else { "FAIL" };
        
        println!("{}[{}]{} Stream Test", get_color(status_color), status, RESET);
        println!("  Input:    {}", self.input);
        println!("  Expected: {}", self.expected);
        println!("  Actual:   {}", self.actual);
    }
}

// Re-export commonly used items for macro convenience
pub use crate::xstream::{MergeStrategy, CollisionPolicy, GateCondition};
pub use crate::adapter::AdapterError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xstream_macro() {
        let input = "ui:theme=\"dark\"; db:host=\"localhost\"";
        let result = xstream!(input).fork_by(&["ui", "db"]).collect();
        
        assert!(result.contains("ui:"));
        assert!(result.contains("db:"));
    }
    
    #[test]
    fn test_fork_colored_macro() {
        let input = "ui:btn=\"click\"; db:host=\"localhost\"; ui:theme=\"dark\"";
        let streams = fork_colored!(input, "ui", "db");
        
        assert_eq!(streams.len(), 2);
        
        // Should have ui stream with both ui tokens (accounting for color codes)
        assert!(streams.iter().any(|s| s.contains("btn=") && s.contains("click") && s.contains("theme=") && s.contains("dark")));
        // Should have db stream with db token (accounting for color codes)  
        assert!(streams.iter().any(|s| s.contains("host=") && s.contains("localhost")));
    }
    
    #[test]
    fn test_pipeline_macro() {
        let input = "ui:a=\"1\"; ui:b=\"2\"; db:x=\"9\"";
        // Basic test without complex chaining for now
        let result = pipeline!(input => fork(["ui", "db"]));
        
        // Should contain both namespaces in fork result
        assert!(result.contains("ui:"));
        assert!(result.contains("db:"));
    }
    
    #[test]
    fn test_generate_stream_macro() {
        let streams = generate_stream!(namespaces: ["ui", "db"], tokens_per_ns: 2, colors: false);
        
        assert_eq!(streams.len(), 2);
        assert!(streams[0].contains("ui:key0=\"value0\""));
        assert!(streams[0].contains("ui:key1=\"value1\""));
        assert!(streams[1].contains("db:key0=\"value2\""));
        assert!(streams[1].contains("db:key1=\"value3\""));
    }
    
    #[test]
    fn test_test_stream_macro() {
        let input = "ui:theme=\"dark\"";
        let result = test_stream!(input, "theme", |s| s.fork_by(&["ui"]));
        
        assert!(result.passed);
        assert!(result.actual.contains("theme=\"dark\""));
    }
    
    #[test]
    fn test_build_pipeline_macro() {
        let pipeline = build_pipeline!(fork => ["ui", "db"]; gate => 1);
        let input = "ui:a=\"1\"; db:b=\"2\"";
        let result = pipeline.execute(input).unwrap();
        
        // Should have forked streams with gate filtering
        assert!(result.contains("ui:"));
        assert!(result.contains("db:"));
    }
    
    #[test]
    fn test_validate_stream_macro() {
        let valid = "key=\"value\"";
        let invalid = "not_valid_format";
        
        assert!(validate_stream!(valid).is_ok());
        assert!(validate_stream!(invalid).is_err());
        
        let custom_error = validate_stream!(invalid => "Custom error message");
        match custom_error {
            Err(AdapterError::InvalidInput(msg)) => assert_eq!(msg, "Custom error message"),
            _ => panic!("Expected custom error message"),
        }
    }
    
    #[test]
    fn test_setup_streams_macro() {
        let streams = setup_streams!(config: 
            ui = "theme=\"dark\"; size=\"large\"",
            db = "host=\"localhost\"; port=\"5432\""
        );
        
        assert_eq!(streams.len(), 2);
        assert_eq!(streams["ui"], "theme=\"dark\"; size=\"large\"");
        assert_eq!(streams["db"], "host=\"localhost\"; port=\"5432\"");
    }
    
    #[test]
    fn test_batch_process_macro() {
        let stream1 = "ui:a=\"1\"";
        let stream2 = "db:b=\"2\"";
        
        let results = batch_process!(streams: [stream1, stream2] => fork(["ui", "db"]));
        
        assert_eq!(results.len(), 2);
        // Both should have fork results
        assert!(results[0].contains("ui:"));
        assert!(results[1].contains("db:"));
    }
    
    #[test]
    fn test_process_if_macro() {
        let ui_stream = "ui:theme=\"dark\"";
        let db_stream = "db:host=\"localhost\"";
        
        let ui_result = process_if!(ui_stream, has_namespace("ui") => fork(["ui"]));
        let db_result = process_if!(db_stream, has_namespace("ui") => fork(["ui"]));
        
        // UI stream should be processed (has ui namespace)
        assert_ne!(ui_result, ui_stream);
        // DB stream should pass through unchanged (no ui namespace)
        assert_eq!(db_result, db_stream);
    }
    
    #[test]
    fn test_pipeline_with_gate() {
        let input = "ui:a=\"1\"; ui:b=\"2\"; db:x=\"9\""; // ui has 2 tokens, db has 1
        let forked = pipeline!(input => fork(["ui", "db"]));
        let gated = pipeline!(forked => gate(min_tokens: 2));
        
        // Should contain ui namespace (2 tokens) but filter out db (1 token)
        assert!(gated.contains("ui:"));
        // db should be filtered out since it only has 1 token
        assert!(!gated.contains("db:"));
    }
}