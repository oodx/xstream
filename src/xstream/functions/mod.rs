// XStream Functions - Main user-facing API
//
// This module provides the primary function-based API for XStream transformations.
// Re-exports RSB's streamable functions for consistency.

// Re-export RSB's function-based interfaces for general text processing
pub use rsb::streamable::{
    replace_fn, uppercase_fn, lowercase_fn, reverse_fn, trim_fn,
    base64_encode_fn, base64_decode_fn, url_encode_fn, url_decode_fn,
};

// === TOKEN-SPECIFIC FUNCTIONS ===
use crate::xstream::types::*;

/// Count tokens in the stream
pub fn token_count_fn(input: &str, _: ()) -> String {
    TokenCount::stream_apply(input, ())
}

/// Extract token keys
pub fn extract_keys_fn(input: &str, _: ()) -> String {
    ExtractKeys::stream_apply(input, ())
}

/// Extract token values
pub fn extract_values_fn(input: &str, _: ()) -> String {
    ExtractValues::stream_apply(input, ())
}

/// Filter tokens based on a pattern
pub fn filter_tokens_fn(input: &str, pattern: String) -> String {
    FilterTokens::stream_apply(input, pattern)
}

/// Extract all namespaces from token stream
pub fn extract_namespaces_fn(input: &str, _: ()) -> String {
    ExtractNamespaces::stream_apply(input, ())
}

/// Filter tokens by namespace
pub fn filter_by_namespace_fn(input: &str, namespace: String) -> String {
    FilterByNamespace::stream_apply(input, namespace)
}

/// Validate token stream format
pub fn validate_tokens_fn(input: &str, _: ()) -> String {
    TokenValidate::stream_apply(input, ())
}