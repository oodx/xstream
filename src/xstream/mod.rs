pub mod types;
pub mod parse;
pub mod gen;
pub mod transform;
pub mod fork;
pub mod merge;
pub mod gate;

// Re-export types for convenience
pub use types::*;
pub use gen::{gen_token, gen_flat_token, gen_ns_token, gen_token_stream, gen_config_stream, ValueType};
pub use transform::{transform, TokenStream, TX};
pub use fork::{fork_by_namespace, fork_all_namespaces, fork_by_pattern, ForkMode};
pub use merge::{merge_concat, merge_with_strategy, merge_with_collision_policy, MergeStrategy, CollisionPolicy, MergeFilter};
pub use gate::{xor_gate, multi_xor_gate, timed_gate, xor_gate_with_state, sync_gate, balance_gate, wait_gate, GateState, GateCondition};

// Re-export token-specific streamables for Pattern 2 composability
pub use types::{
    TokenCount, ExtractKeys, ExtractValues, FilterTokens,
    ExtractNamespaces, FilterByNamespace, TokenValidate,
    TokenToLines, LinesToTokens,
};