pub mod xstream;
pub mod colors;

// Re-export everything from xstream submodules
pub use xstream::types::*;
pub use xstream::gen::*;
pub use xstream::transform::{TokenStream, transform, TX};
pub use xstream::fork::*;
pub use xstream::merge::*;
pub use xstream::gate::*;