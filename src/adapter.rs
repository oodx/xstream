//! XStream Integration Adapter
//! 
//! Provides easy integration patterns for xstream with other systems.
//! Focuses on user ergonomics and common use cases.

use rsb::prelude::*;
use crate::xstream::*;
use crate::xstream::types::{TokenBucket, BucketMode, is_token_streamable};
use crate::colors::{get_channel_color_name, pre_color_stream};
use std::collections::HashMap;
use serde_json;

/// Error types for adapter operations
#[derive(Debug, Clone)]
pub enum AdapterError {
    InvalidInput(String),
    ParseError(String),
    SerializationError(String),
    ProcessingError(String),
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AdapterError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AdapterError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            AdapterError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
        }
    }
}

impl std::error::Error for AdapterError {}

/// Simple adapter for common xstream operations
pub struct XStreamAdapter {
    /// Internal state for chaining operations
    current_stream: Option<String>,
    namespace_colors: HashMap<String, usize>,
    color_enabled: bool,
}

impl XStreamAdapter {
    /// Create a new adapter instance
    pub fn new() -> Self {
        Self {
            current_stream: None,
            namespace_colors: HashMap::new(),
            color_enabled: true,
        }
    }
    
    /// Create adapter with color support disabled
    pub fn new_no_color() -> Self {
        Self {
            current_stream: None,
            namespace_colors: HashMap::new(),
            color_enabled: false,
        }
    }
    
    /// Easy stream processing with method chaining
    pub fn process_stream(&mut self, input: &str) -> StreamProcessor {
        self.current_stream = Some(input.to_string());
        StreamProcessor::new(input, self.color_enabled)
    }
    
    /// Integration with external data sources - JSON to token stream
    pub fn from_json(&mut self, json: &str) -> Result<String, AdapterError> {
        let value: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| AdapterError::ParseError(format!("JSON parse error: {}", e)))?;
        
        let mut tokens = Vec::new();
        self.json_to_tokens(&value, None, &mut tokens)?;
        
        if tokens.is_empty() {
            return Err(AdapterError::InvalidInput("Empty JSON object".to_string()));
        }
        
        let stream = tokens.join("; ");
        self.current_stream = Some(stream.clone());
        Ok(stream)
    }
    
    /// Integration with CSV data - convert CSV to token stream
    pub fn from_csv(&mut self, csv: &str) -> Result<String, AdapterError> {
        let lines: Vec<&str> = csv.trim().lines().collect();
        if lines.len() < 2 {
            return Err(AdapterError::InvalidInput("CSV must have header and at least one data row".to_string()));
        }
        
        let headers: Vec<&str> = lines[0].split(',').map(|h| h.trim()).collect();
        let mut all_tokens = Vec::new();
        
        for (row_idx, line) in lines[1..].iter().enumerate() {
            let values: Vec<&str> = line.split(',').map(|v| v.trim()).collect();
            if values.len() != headers.len() {
                continue; // Skip malformed rows
            }
            
            // Create namespace for each row
            let namespace = format!("row{}", row_idx);
            for (header, value) in headers.iter().zip(values.iter()) {
                if !header.is_empty() && !value.is_empty() {
                    all_tokens.push(format!("{}:{}=\"{}\"", namespace, header, value));
                }
            }
        }
        
        if all_tokens.is_empty() {
            return Err(AdapterError::InvalidInput("No valid CSV data found".to_string()));
        }
        
        let stream = all_tokens.join("; ");
        self.current_stream = Some(stream.clone());
        Ok(stream)
    }
    
    /// Convert token stream back to JSON
    pub fn to_json(&self, stream: &str) -> Result<String, AdapterError> {
        if !is_token_streamable(stream) {
            return Err(AdapterError::InvalidInput("Invalid token stream format".to_string()));
        }
        
        let bucket = TokenBucket::from_str(stream, BucketMode::Hybrid)
            .map_err(|e| AdapterError::ParseError(format!("Token parse error: {}", e)))?;
        
        let json_value = self.bucket_to_json(&bucket)?;
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| AdapterError::SerializationError(format!("JSON serialization error: {}", e)))
    }
    
    /// Common pattern: split stream by namespaces and process each
    pub fn split_and_process(&mut self, input: &str, namespaces: &[&str]) -> Vec<String> {
        if !is_token_streamable(input) {
            return vec![];
        }
        
        let fork_result = input.to_string().stream_apply(Fork, 
            namespaces.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        
        // Parse the fork result - each line is "namespace: tokens"
        fork_result.lines()
            .filter_map(|line| {
                if let Some((ns, tokens)) = line.split_once(": ") {
                    if self.color_enabled {
                        let color_idx = namespaces.iter().position(|&n| n == ns).unwrap_or(0);
                        self.namespace_colors.insert(ns.to_string(), color_idx);
                        Some(pre_color_stream(tokens, get_channel_color_name(color_idx)))
                    } else {
                        Some(tokens.to_string())
                    }
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Common pattern: merge multiple streams with filtering
    pub fn merge_and_filter(&mut self, streams: &[&str], min_tokens: usize) -> String {
        // Filter streams by minimum token count
        let filtered_streams: Vec<&str> = streams.iter()
            .filter(|&stream| {
                if !is_token_streamable(stream) {
                    return false;
                }
                let token_count = stream.split(';').count();
                token_count >= min_tokens
            })
            .copied()
            .collect();
        
        if filtered_streams.is_empty() {
            return String::new();
        }
        
        merge_concat(&filtered_streams)
    }
    
    /// Get current stream if available
    pub fn current(&self) -> Option<&String> {
        self.current_stream.as_ref()
    }
    
    /// Clear current stream state
    pub fn clear(&mut self) {
        self.current_stream = None;
        self.namespace_colors.clear();
    }
    
    // Helper methods for JSON conversion
    fn json_to_tokens(&self, value: &serde_json::Value, namespace: Option<&str>, tokens: &mut Vec<String>) -> Result<(), AdapterError> {
        match value {
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    let full_key = if let Some(ns) = namespace {
                        format!("{}:{}", ns, key)
                    } else {
                        key.clone()
                    };
                    
                    match val {
                        serde_json::Value::String(s) => {
                            tokens.push(format!("{}=\"{}\"", full_key, s));
                        }
                        serde_json::Value::Number(n) => {
                            tokens.push(format!("{}=\"{}\"", full_key, n));
                        }
                        serde_json::Value::Bool(b) => {
                            tokens.push(format!("{}=\"{}\"", full_key, b));
                        }
                        serde_json::Value::Object(_) => {
                            // Nested object becomes new namespace
                            self.json_to_tokens(val, Some(key), tokens)?;
                        }
                        serde_json::Value::Array(arr) => {
                            // Arrays become indexed tokens
                            for (idx, item) in arr.iter().enumerate() {
                                let array_key = format!("{}[{}]", full_key, idx);
                                match item {
                                    serde_json::Value::String(s) => {
                                        tokens.push(format!("{}=\"{}\"", array_key, s));
                                    }
                                    serde_json::Value::Number(n) => {
                                        tokens.push(format!("{}=\"{}\"", array_key, n));
                                    }
                                    serde_json::Value::Bool(b) => {
                                        tokens.push(format!("{}=\"{}\"", array_key, b));
                                    }
                                    _ => {
                                        // Skip complex nested arrays/objects for now
                                    }
                                }
                            }
                        }
                        serde_json::Value::Null => {
                            tokens.push(format!("{}=\"null\"", full_key));
                        }
                    }
                }
            }
            _ => {
                return Err(AdapterError::InvalidInput("JSON must be an object at root level".to_string()));
            }
        }
        Ok(())
    }
    
    fn bucket_to_json(&self, bucket: &TokenBucket) -> Result<serde_json::Value, AdapterError> {
        let mut json_obj = serde_json::Map::new();
        
        for (namespace, tokens) in &bucket.data {
            if namespace == "global" {
                // Global tokens go to root level
                for (key, value) in tokens {
                    json_obj.insert(key.clone(), serde_json::Value::String(value.clone()));
                }
            } else {
                // Namespace tokens become nested objects
                let mut ns_obj = serde_json::Map::new();
                for (key, value) in tokens {
                    ns_obj.insert(key.clone(), serde_json::Value::String(value.clone()));
                }
                json_obj.insert(namespace.clone(), serde_json::Value::Object(ns_obj));
            }
        }
        
        Ok(serde_json::Value::Object(json_obj))
    }
}

impl Default for XStreamAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Fluent API for stream processing
pub struct StreamProcessor {
    current_stream: String,
    operations: Vec<Operation>,
    color_enabled: bool,
}

#[derive(Debug, Clone)]
enum Operation {
    Fork(Vec<String>),
    Gate(GateCondition),
    Merge(MergeStrategy),
    Transform(String),
}

impl StreamProcessor {
    fn new(stream: &str, color_enabled: bool) -> Self {
        Self {
            current_stream: stream.to_string(),
            operations: Vec::new(),
            color_enabled,
        }
    }
    
    /// Fork stream by namespaces
    pub fn fork_by(&mut self, namespaces: &[&str]) -> &mut Self {
        self.operations.push(Operation::Fork(
            namespaces.iter().map(|s| s.to_string()).collect()
        ));
        self
    }
    
    /// Apply gate condition (minimum tokens per stream)
    pub fn gate_min_tokens(&mut self, min: usize) -> &mut Self {
        self.operations.push(Operation::Gate(GateCondition::MinTokens(min)));
        self
    }
    
    /// Merge with strategy
    pub fn merge_with(&mut self, strategy: MergeStrategy) -> &mut Self {
        self.operations.push(Operation::Merge(strategy));
        self
    }
    
    /// Apply custom transformation
    pub fn transform(&mut self, operation: &str) -> &mut Self {
        self.operations.push(Operation::Transform(operation.to_string()));
        self
    }
    
    /// Execute all operations and collect result
    pub fn collect(&self) -> String {
        let mut current = self.current_stream.clone();
        
        for operation in &self.operations {
            match operation {
                Operation::Fork(namespaces) => {
                    current = current.stream_apply(Fork, namespaces.clone());
                }
                Operation::Gate(condition) => {
                    // Apply gate to each forked stream
                    let lines: Vec<&str> = current.lines().collect();
                    let mut filtered_lines = Vec::new();
                    
                    for line in lines {
                        if let Some((_ns, tokens)) = line.split_once(": ") {
                            let token_count = tokens.split(';').count();
                            match condition {
                                GateCondition::MinTokens(min) => {
                                    if token_count >= *min {
                                        filtered_lines.push(line);
                                    }
                                }
                                // Add more gate conditions as needed
                                _ => filtered_lines.push(line),
                            }
                        }
                    }
                    current = filtered_lines.join("\n");
                }
                Operation::Merge(strategy) => {
                    current = current.stream_apply(Merge, strategy.clone());
                }
                Operation::Transform(_op) => {
                    // Custom transformations can be added here
                    // For now, just pass through
                }
            }
        }
        
        current
    }
    
    /// Collect with color visualization if enabled
    pub fn collect_with_colors(&self) -> String {
        if !self.color_enabled {
            return self.collect();
        }
        
        let result = self.collect();
        
        // Apply colors based on namespace patterns
        if let Ok(bucket) = TokenBucket::from_str(&result, BucketMode::Flat) {
            let colors = ["red", "blue", "green", "orange", "purple", "cyan", "yellow"];
            let mut colored_streams = Vec::new();
            
            for (idx, (namespace, tokens)) in bucket.data.iter().enumerate() {
                let color = colors[idx % colors.len()];
                let token_string = tokens.iter()
                    .map(|(k, v)| format!("{}:{}=\"{}\"", namespace, k, v))
                    .collect::<Vec<_>>()
                    .join("; ");
                colored_streams.push(pre_color_stream(&token_string, color));
            }
            
            colored_streams.join("; ")
        } else {
            result
        }
    }
}

/// Quick builders for common patterns
impl XStreamAdapter {
    /// Builder: Create a simple fork-process-merge pipeline
    pub fn pipeline() -> PipelineBuilder {
        PipelineBuilder::new()
    }
    
    /// Builder: Create colored test streams for visualization
    pub fn test_streams(namespaces: &[&str], tokens_per_ns: usize) -> Vec<String> {
        let colors = ["red", "blue", "green", "orange", "purple", "cyan", "yellow"];
        namespaces.iter()
            .enumerate()
            .map(|(idx, &ns)| {
                let color = colors[idx % colors.len()];
                let tokens: Vec<String> = (0..tokens_per_ns)
                    .map(|i| format!("{}:key{}=\"value{}\"", ns, i, i))
                    .collect();
                let stream = tokens.join("; ");
                pre_color_stream(&stream, color)
            })
            .collect()
    }
    
    /// Quick validation of token stream format
    pub fn validate_stream(stream: &str) -> Result<(), AdapterError> {
        if !is_token_streamable(stream) {
            return Err(AdapterError::InvalidInput("Invalid token stream format".to_string()));
        }
        Ok(())
    }
}

/// Builder for pipeline operations
pub struct PipelineBuilder {
    steps: Vec<PipelineStep>,
}

#[derive(Debug, Clone)]
enum PipelineStep {
    Fork(Vec<String>),
    Gate(usize),
    Merge(MergeStrategy),
    Filter(String),
}

impl PipelineBuilder {
    fn new() -> Self {
        Self {
            steps: Vec::new(),
        }
    }
    
    pub fn fork(mut self, namespaces: &[&str]) -> Self {
        self.steps.push(PipelineStep::Fork(
            namespaces.iter().map(|s| s.to_string()).collect()
        ));
        self
    }
    
    pub fn gate(mut self, min_tokens: usize) -> Self {
        self.steps.push(PipelineStep::Gate(min_tokens));
        self
    }
    
    pub fn merge(mut self, strategy: MergeStrategy) -> Self {
        self.steps.push(PipelineStep::Merge(strategy));
        self
    }
    
    pub fn filter(mut self, condition: &str) -> Self {
        self.steps.push(PipelineStep::Filter(condition.to_string()));
        self
    }
    
    pub fn execute(&self, input: &str) -> Result<String, AdapterError> {
        XStreamAdapter::validate_stream(input)?;
        
        let mut current = input.to_string();
        
        for step in &self.steps {
            match step {
                PipelineStep::Fork(namespaces) => {
                    current = current.stream_apply(Fork, namespaces.clone());
                }
                PipelineStep::Gate(min_tokens) => {
                    // Filter forked streams by token count
                    let lines: Vec<&str> = current.lines().collect();
                    let filtered: Vec<&str> = lines.into_iter()
                        .filter(|line| {
                            if let Some((_, tokens)) = line.split_once(": ") {
                                tokens.split(';').count() >= *min_tokens
                            } else {
                                false
                            }
                        })
                        .collect();
                    current = filtered.join("\n");
                }
                PipelineStep::Merge(strategy) => {
                    current = current.stream_apply(Merge, strategy.clone());
                }
                PipelineStep::Filter(_condition) => {
                    // Custom filtering can be implemented here
                }
            }
        }
        
        Ok(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_json_conversion() {
        let mut adapter = XStreamAdapter::new();
        
        let json = r#"{"host": "localhost", "port": 8080, "db": {"user": "admin", "pass": "secret"}}"#;
        let stream = adapter.from_json(json).unwrap();
        
        assert!(stream.contains("host=\"localhost\""));
        assert!(stream.contains("port=\"8080\""));
        assert!(stream.contains("db:user=\"admin\""));
        assert!(stream.contains("db:pass=\"secret\""));
        
        // Convert back to JSON
        let json_back = adapter.to_json(&stream).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_back).unwrap();
        
        assert_eq!(parsed["host"], "localhost");
        assert_eq!(parsed["port"], "8080");
        assert_eq!(parsed["db"]["user"], "admin");
    }
    
    #[test]
    fn test_adapter_csv_conversion() {
        let mut adapter = XStreamAdapter::new();
        
        let csv = "name,age,city\nAlice,25,NYC\nBob,30,SF";
        let stream = adapter.from_csv(csv).unwrap();
        
        assert!(stream.contains("row0:name=\"Alice\""));
        assert!(stream.contains("row0:age=\"25\""));
        assert!(stream.contains("row1:name=\"Bob\""));
        assert!(stream.contains("row1:city=\"SF\""));
    }
    
    #[test]
    fn test_fluent_processor() {
        let mut adapter = XStreamAdapter::new();
        let input = "ui:theme=\"dark\"; ui:size=\"large\"; db:host=\"localhost\"; db:port=\"5432\"";
        
        let result = adapter.process_stream(input)
            .fork_by(&["ui", "db"])
            .gate_min_tokens(2)
            .merge_with(MergeStrategy::Sort)
            .collect();
        
        // Both ui and db namespaces have 2 tokens, so both should pass the gate
        assert!(result.contains("ui:theme"));
        assert!(result.contains("db:host"));
    }
    
    #[test]
    fn test_pipeline_builder() {
        let pipeline = XStreamAdapter::pipeline()
            .fork(&["ui", "db", "log"])
            .gate(1)
            .merge(MergeStrategy::Concat);
        
        let input = "ui:theme=\"dark\"; db:host=\"localhost\"; log:level=\"info\"";
        let result = pipeline.execute(input).unwrap();
        
        assert!(result.contains("theme=\"dark\""));
        assert!(result.contains("host=\"localhost\""));
        assert!(result.contains("level=\"info\""));
    }
    
    #[test]
    fn test_split_and_process() {
        let mut adapter = XStreamAdapter::new_no_color(); // Disable colors for testing
        let input = "ui:btn=\"click\"; db:host=\"localhost\"; ui:theme=\"dark\"";
        
        let processed = adapter.split_and_process(input, &["ui", "db"]);
        
        assert_eq!(processed.len(), 2);
        // UI stream should have 2 tokens
        assert!(processed.iter().any(|s| s.contains("btn=\"click\"") && s.contains("theme=\"dark\"")));
        // DB stream should have 1 token
        assert!(processed.iter().any(|s| s.contains("host=\"localhost\"")));
    }
    
    #[test]
    fn test_merge_and_filter() {
        let mut adapter = XStreamAdapter::new();
        
        let streams = &[
            "a=\"1\"",                    // 1 token - should be filtered out
            "b=\"2\"; c=\"3\"",           // 2 tokens - should pass
            "d=\"4\"; e=\"5\"; f=\"6\"",  // 3 tokens - should pass
        ];
        
        let result = adapter.merge_and_filter(streams, 2);
        
        assert!(!result.contains("a=\"1\""));  // Filtered out
        assert!(result.contains("b=\"2\""));   // Included
        assert!(result.contains("d=\"4\""));   // Included
    }
}