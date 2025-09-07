// XStream Functions - Main user-facing API
//
// This module provides the primary function-based API for XStream transformations.
// Re-exports RSB's streamable functions for consistency.

// Re-export RSB's function-based interfaces
pub use rsb::streamable::{replace_fn, uppercase_fn, lowercase_fn};

// Additional function-based interfaces using RSB streamables
use rsb::streamable::*;

/// Reverse the text
pub fn reverse_fn(input: &str, _: ()) -> String {
    Reverse::stream_apply(input, ())
}

/// Trim whitespace from both ends  
pub fn trim_fn(input: &str, _: ()) -> String {
    Trim::stream_apply(input, ())
}

/// Base64 encode the input
pub fn base64_encode_fn(input: &str, _: ()) -> String {
    Base64Encode::stream_apply(input, ())
}

/// Base64 decode the input
pub fn base64_decode_fn(input: &str, _: ()) -> String {
    Base64Decode::stream_apply(input, ())
}

/// URL encode the input
pub fn url_encode_fn(input: &str, _: ()) -> String {
    UrlEncode::stream_apply(input, ())
}

/// URL decode the input
pub fn url_decode_fn(input: &str, _: ()) -> String {
    UrlDecode::stream_apply(input, ())
}

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