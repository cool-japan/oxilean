//! # FutharkReverse - Trait Implementations
//!
//! This module contains trait implementations for `FutharkReverse`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkReverse;
use std::fmt;

impl std::fmt::Display for FutharkReverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "reverse {}", self.array)
    }
}
