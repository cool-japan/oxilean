//! # StackOverflow - Trait Implementations
//!
//! This module contains trait implementations for `StackOverflow`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StackOverflow;
use std::fmt;

impl fmt::Display for StackOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "stack overflow: depth {} exceeds maximum {}",
            self.depth, self.max_depth
        )
    }
}

impl std::error::Error for StackOverflow {}
