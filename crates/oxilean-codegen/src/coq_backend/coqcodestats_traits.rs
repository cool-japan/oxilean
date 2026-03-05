//! # CoqCodeStats - Trait Implementations
//!
//! This module contains trait implementations for `CoqCodeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqCodeStats;
use std::fmt;

impl std::fmt::Display for CoqCodeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CoqCodeStats {{ theorems={}, defs={}, lemmas={}, axioms={}, lines={} }}",
            self.theorems, self.definitions, self.lemmas, self.axioms, self.total_lines
        )
    }
}
