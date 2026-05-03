//! Hole inference: automatically fill `_` (holes) in terms based on type constraints.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
