//! Dead-declaration lint module.
//!
//! Detects OxiLean declarations (theorems, lemmas, defs, axioms) that are
//! defined but never referenced within the current file or module.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
