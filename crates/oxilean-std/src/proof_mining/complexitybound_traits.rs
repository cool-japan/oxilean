//! # ComplexityBound - Trait Implementations
//!
//! This module contains trait implementations for `ComplexityBound`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ComplexityBound;
use std::fmt;

impl std::fmt::Display for ComplexityBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexityBound::Constant => write!(f, "O(1)"),
            ComplexityBound::Linear => write!(f, "O(n)"),
            ComplexityBound::Polynomial(k) => write!(f, "O(n^{})", k),
            ComplexityBound::Exponential => write!(f, "O(2^n)"),
            ComplexityBound::NonElementary => write!(f, "non-elementary"),
        }
    }
}
