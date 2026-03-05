//! # FinSurreal - Trait Implementations
//!
//! This module contains trait implementations for `FinSurreal`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FinSurreal;
use std::fmt;

impl std::fmt::Display for FinSurreal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.exp == 0 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, 1u64 << self.exp)
        }
    }
}
