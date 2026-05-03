//! Algebraic Simplification optimisation pass.
//!
//! Simplifies algebraic expressions using mathematical identities such as
//! `x + 0 = x`, `x * 1 = x`, `x - x = 0`, constant folding, and more.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
