//! # CoqHypothesis - Trait Implementations
//!
//! This module contains trait implementations for `CoqHypothesis`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqHypothesis;
use std::fmt;

impl std::fmt::Display for CoqHypothesis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hypothesis {} : {}.", self.name, self.hyp_type)
    }
}
