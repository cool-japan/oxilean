//! # UnificationStats - Trait Implementations
//!
//! This module contains trait implementations for `UnificationStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::UnificationStats;

impl std::fmt::Display for UnificationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UnifStats {{ ok: {}, fail: {}, postponed: {}, steps: {} }}",
            self.successes, self.failures, self.postponed, self.steps
        )
    }
}
