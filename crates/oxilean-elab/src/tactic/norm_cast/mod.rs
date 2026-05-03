//! `norm_cast` tactic — normalises coercions in goals and hypotheses.
//!
//! This module provides the data types and algorithms required to push type
//! casts into a canonical (usually inward) position, simplifying goals that
//! mix numeric types (e.g. `Nat`, `Int`, `Real`) via coercion functions.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
