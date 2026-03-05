//! # DocComment - Trait Implementations
//!
//! This module contains trait implementations for `DocComment`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DocComment;
use std::fmt;

impl fmt::Display for DocComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/-- {} -/", self.text.trim())
    }
}
