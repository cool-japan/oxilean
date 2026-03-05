//! # CoqSolveObligations - Trait Implementations
//!
//! This module contains trait implementations for `CoqSolveObligations`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqSolveObligations;
use std::fmt;

impl std::fmt::Display for CoqSolveObligations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(t) = &self.tactic {
            write!(f, "Solve Obligations with {}.", t)
        } else {
            write!(f, "Solve All Obligations.")
        }
    }
}
