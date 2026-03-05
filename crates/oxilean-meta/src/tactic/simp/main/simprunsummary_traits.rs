//! # SimpRunSummary - Trait Implementations
//!
//! This module contains trait implementations for `SimpRunSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SimpRunSummary;

impl std::fmt::Display for SimpRunSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SimpRunSummary {{ runs: {}, effective: {}, proved: {} }}",
            self.num_runs,
            self.effective_runs(),
            self.proved_runs
        )
    }
}
