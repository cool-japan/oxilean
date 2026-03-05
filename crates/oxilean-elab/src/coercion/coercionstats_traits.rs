//! # CoercionStats - Trait Implementations
//!
//! This module contains trait implementations for `CoercionStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoercionStats;
use std::fmt;

impl std::fmt::Display for CoercionStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CoercionStats {{ inserted: {}, failed: {}, chained: {}, sort: {} }}",
            self.inserted, self.failed, self.chained, self.sort_coercions
        )
    }
}
