//! Universe level management and polymorphism.
//!
//! This module provides the data structures and algorithms for Lean-4-style
//! universe levels, including normalisation, comparison, unification, and
//! constraint solving.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
