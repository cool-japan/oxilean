//! # IoError - Trait Implementations
//!
//! This module contains trait implementations for `IoError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IoError;
use std::fmt;

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IO error ({}): {}", self.kind, self.message)
    }
}
