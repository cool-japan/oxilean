//! # FutharkCopy - Trait Implementations
//!
//! This module contains trait implementations for `FutharkCopy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkCopy;
use std::fmt;

impl std::fmt::Display for FutharkCopy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "copy {}", self.array)
    }
}
