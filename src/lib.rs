pub mod xstream;
pub mod colors;
pub mod adapter;
pub mod macros;

// Re-export everything from xstream submodules
pub use xstream::types::*;
pub use xstream::gen::*;
pub use xstream::transform::{TokenStream, transform, TX};
pub use xstream::fork::*;
pub use xstream::merge::*;
pub use xstream::gate::*;

// Ensure streamable structs are accessible
pub use xstream::{Fork, ForkAll, ForkPattern, Merge, SelectiveMerge, WeightedMerge};

// Re-export user-friendly integration components
pub use adapter::{XStreamAdapter, StreamProcessor, AdapterError, PipelineBuilder};
pub use macros::TestResult;