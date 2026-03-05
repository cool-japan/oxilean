//! # CoqTacticNotation - Trait Implementations
//!
//! This module contains trait implementations for `CoqTacticNotation`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqTacticNotation;
use std::fmt;

impl std::fmt::Display for CoqTacticNotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pat = self.pattern.join(" ");
        write!(
            f,
            "Tactic Notation (at level {}) {} := {}.",
            self.level, pat, self.body
        )
    }
}
