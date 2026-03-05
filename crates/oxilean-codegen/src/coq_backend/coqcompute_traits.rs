//! # CoqCompute - Trait Implementations
//!
//! This module contains trait implementations for `CoqCompute`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqCompute;
use std::fmt;

impl std::fmt::Display for CoqCompute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compute {}.", self.expr)
    }
}
