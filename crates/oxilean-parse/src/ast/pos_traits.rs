//! # Pos - Trait Implementations
//!
//! This module contains trait implementations for `Pos`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Pos;
use std::fmt;

impl Default for Pos {
    fn default() -> Self {
        Self::start()
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
