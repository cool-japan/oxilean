//! # SourceRange - Trait Implementations
//!
//! This module contains trait implementations for `SourceRange`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;

use super::types::SourceRange;

impl fmt::Display for SourceRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref file) = self.file {
            write!(f, "{}:{}..{}", file, self.start, self.end)
        } else {
            write!(f, "{}..{}", self.start, self.end)
        }
    }
}
