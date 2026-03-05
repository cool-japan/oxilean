//! # MacroError - Trait Implementations
//!
//! This module contains trait implementations for `MacroError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MacroError;
use std::fmt;

impl fmt::Display for MacroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "macro error ({}): {}", self.kind, self.message)
    }
}

impl std::error::Error for MacroError {}
