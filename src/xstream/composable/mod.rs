// XStream Composable - RSB streamable integration for power users
//
// This module re-exports RSB's streamable functionality for advanced usage,
// composition, and custom pipeline construction.
//
// Most users should use the main xstream API. Use this for:
// - Building custom transformations
// - Direct struct-based pipelines  
// - Extending with custom streamable functions

// Re-export RSB streamable functionality (unix-style, encoding, etc.)
pub use rsb::streamable::{
    Streamable, StreamApply,
    // Basic text transforms
    Replace, UpperCase, LowerCase, Reverse, Trim,
    // Unix-style streamables
    Head, Tail, Grep, Sort, Unique, WordCount,
    // Encoding transforms  
    Base64Encode, Base64Decode, UrlEncode, UrlDecode,
    // RSB integration streamables
    Sed, SedLines,
};

// Re-export xstream's token-specific streamables
pub use crate::xstream::types::{
    TokenCount, ExtractKeys, ExtractValues, FilterTokens,
    ExtractNamespaces, FilterByNamespace, TokenValidate,
    TokenToLines, LinesToTokens,
};

// Re-export the streamable! macro from RSB
pub use rsb::streamable;