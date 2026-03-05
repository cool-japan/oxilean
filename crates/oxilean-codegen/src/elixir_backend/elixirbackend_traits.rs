//! # ElixirBackend - Trait Implementations
//!
//! This module contains trait implementations for `ElixirBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElixirBackend;
use std::fmt;

impl Default for ElixirBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ElixirBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ElixirBackend")
    }
}
