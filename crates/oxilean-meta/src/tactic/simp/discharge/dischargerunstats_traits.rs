//! # DischargeRunStats - Trait Implementations
//!
//! This module contains trait implementations for `DischargeRunStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DischargeRunStats;

impl std::fmt::Display for DischargeRunStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DischargeRunStats {{ successes: {}, failures: {} }}",
            self.successes, self.failures
        )
    }
}
