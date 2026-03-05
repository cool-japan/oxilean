//! # CoherenceViolation - Trait Implementations
//!
//! This module contains trait implementations for `CoherenceViolation`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoherenceViolation;
use std::fmt;

impl std::fmt::Display for CoherenceViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "coherence violation for class {}: {}",
            self.class, self.message
        )
    }
}
