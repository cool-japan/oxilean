//! # Comparison - Trait Implementations
//!
//! This module contains trait implementations for `Comparison`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Comparison;
use std::fmt;

impl std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = if self.is_improvement() {
            "faster"
        } else if self.is_regression() {
            "slower"
        } else {
            "same"
        };
        write!(
            f,
            "{} vs {}: {:.2}x {} ({:+.1}%, {:+.1} ns)",
            self.baseline_name,
            self.candidate_name,
            self.speedup,
            direction,
            self.pct_change,
            self.diff_ns,
        )
    }
}
