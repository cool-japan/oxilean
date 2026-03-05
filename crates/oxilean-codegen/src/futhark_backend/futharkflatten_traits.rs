//! # FutharkFlatten - Trait Implementations
//!
//! This module contains trait implementations for `FutharkFlatten`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkFlatten;
use std::fmt;

impl std::fmt::Display for FutharkFlatten {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "flatten {}", self.array)
    }
}
