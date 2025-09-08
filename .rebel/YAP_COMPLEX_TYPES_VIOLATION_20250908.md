# 🦊 RSB VIOLATION YAP
**Date**: 2025-09-08
**Target**: /home/xnull/repos/code/rust/oodx/xstream/src/xstream/types/streamable.rs
**Violation Type**: Complex types in public function signatures - Breaking String-First Philosophy

## VIOLATION DETECTED 🚨
```rust
// streamable.rs lines 31-41
streamable!(FilterTokens(stdin, key_contains: String) => {  // ❌ Complex streamable signature
    stdin.split(';')
        .filter(|token| token.contains(&key_contains))
        .collect::<Vec<_>>()  // ❌ Vec collection instead of string-first
        .join("; ")
});

// lines 56-83
streamable!(FilterByNamespace(stdin, namespace: String) => {
    let mut current_ns = "global".to_string();  // ❌ Mutable state instead of string flow
    let mut result = Vec::new();  // ❌ Vec accumulator instead of stream processing
    
    for token in stdin.split(';') {  // ❌ Manual iteration instead of RSB stream ops
        // ... complex logic
    }
    
    result.join("; ")
});
```

## CANONICAL RSB PATTERN 📚
From RSB Architecture lines 32-40:
```rust
// ✅ RSB Pattern: String-biased signatures
pub fn read_config(path: &str) -> String;
pub fn process_logs(input: &str, pattern: &str) -> String;
pub fn send_alert(message: &str, recipient: &str) -> i32;

// ❌ Anti-Pattern: Complex type signatures
pub fn process<T, E>(input: Result<Option<T>, E>) -> Result<Vec<Config>, ProcessError>
where T: Deserialize + Clone, E: Error + Send;
```

From RSB Architecture lines 64-84 - Stream processing benefits:
```rust
// RSB: Unix pipe-like operations (familiar mental model)
let results = cat!("access.log")
    .grep("ERROR")
    .cut(4, " ")
    .sort()
    .uniq()
    .to_string();
```

## CORRECTIVE ACTION ⚡
Replace complex streamables with RSB string-first functions:

```rust
// ✅ RSB COMPLIANT VERSION
use rsb::prelude::*;

/// Filter tokens by key pattern - RSB string-first approach
pub fn filter_tokens_by_key(input: &str, key_pattern: &str) -> String {
    pipe!(input)
        .grep(key_pattern)  // Use RSB stream operations
        .to_string()
}

/// Filter tokens by namespace - Unix pipe approach
pub fn filter_by_namespace(input: &str, target_namespace: &str) -> String {
    let namespace_pattern = format!("{}:", target_namespace);
    pipe!(input)
        .grep(&namespace_pattern)
        .to_string()
}

/// Extract token keys - string-first processing
pub fn extract_token_keys(input: &str) -> String {
    pipe!(input)
        .sed(r"=.*", "")  // Remove values, keep only keys
        .sed(";", "\n")   // Convert to line-based format
        .to_string()
}

/// Extract token values - RSB stream processing
pub fn extract_token_values(input: &str) -> String {
    pipe!(input)
        .sed(r"[^=]*=", "")      // Remove keys, keep only values
        .sed("\"", "")           // Remove quotes
        .sed(";", "\n")          // Convert to line format
        .to_string()
}

/// Count tokens - simple string processing
pub fn count_tokens(input: &str) -> String {
    let count = input.split(';')
        .filter(|s| !s.trim().is_empty())
        .count();
    count.to_string()
}

/// Validate token stream format
pub fn validate_token_stream(input: &str) -> String {
    use crate::xstream::types::is_token_streamable;
    if is_token_streamable(input) {
        "valid"
    } else {
        "invalid"
    }.to_string()
}
```

## REFERENCE 📖
RSB Architecture Section 1.2: "String-Biased Philosophy"
- Lines 32-40: String-biased signatures are the RSB way
- Lines 44-62: String bias benefits - familiar, composable, debuggable
- Lines 64-84: Stream processing with Unix pipe mental model
- Lines 656-674: String processing patterns vs complex type anti-patterns