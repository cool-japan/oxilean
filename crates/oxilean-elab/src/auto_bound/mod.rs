//! Auto-bound implicit variable handling.
//!
//! Lean 4 automatically adds implicit parameters for free names that appear in
//! a declaration's type but are not declared in its parameter list.  This
//! module provides the data types and algorithms for that transformation.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
