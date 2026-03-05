//! # FutharkConcat - Trait Implementations
//!
//! This module contains trait implementations for `FutharkConcat`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkConcat;
use std::fmt;

impl std::fmt::Display for FutharkConcat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrays = self.arrays.join(" ");
        if let Some(d) = self.dim {
            write!(f, "concat @{} {}", d, arrays)
        } else {
            write!(f, "concat {}", arrays)
        }
    }
}
