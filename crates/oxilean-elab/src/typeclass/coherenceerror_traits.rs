//! # CoherenceError - Trait Implementations
//!
//! This module contains trait implementations for `CoherenceError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoherenceError;
use std::fmt;

impl std::fmt::Display for CoherenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoherenceError::OverlappingInstances(a, b) => {
                write!(f, "overlapping instances: {} and {}", a, b)
            }
            CoherenceError::OrphanInstance(n) => write!(f, "orphan instance: {}", n),
            CoherenceError::DuplicateInstance(n) => {
                write!(f, "duplicate instance: {}", n)
            }
        }
    }
}
