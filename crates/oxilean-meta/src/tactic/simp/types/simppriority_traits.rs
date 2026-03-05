//! # SimpPriority - Trait Implementations
//!
//! This module contains trait implementations for `SimpPriority`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SimpPriority;

impl Default for SimpPriority {
    fn default() -> Self {
        SimpPriority::DEFAULT
    }
}

impl std::fmt::Display for SimpPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
