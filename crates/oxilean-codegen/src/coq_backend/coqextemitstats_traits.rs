//! # CoqExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `CoqExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqExtEmitStats;
use std::fmt;

impl std::fmt::Display for CoqExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CoqExtEmitStats {{ theorems={}, defs={}, lemmas={}, axioms={}, tactics={} }}",
            self.theorems_emitted,
            self.definitions_emitted,
            self.lemmas_emitted,
            self.axioms_emitted,
            self.tactics_emitted,
        )
    }
}
