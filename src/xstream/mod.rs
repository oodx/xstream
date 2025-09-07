pub mod types;
pub mod parse;
pub mod gen;
pub mod transform;

// Re-export types for convenience
pub use types::*;
pub use gen::{gen_token, gen_flat_token, gen_ns_token, gen_token_stream, gen_config_stream, ValueType};
pub use transform::{transform, TokenStream, tx};

// Re-export token-specific streamables for Pattern 2 composability
pub use types::{
    TokenCount, ExtractKeys, ExtractValues, FilterTokens,
    ExtractNamespaces, FilterByNamespace, TokenValidate,
    TokenToLines, LinesToTokens,
};