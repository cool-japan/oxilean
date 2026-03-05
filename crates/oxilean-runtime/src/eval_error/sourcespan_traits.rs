//! # SourceSpan - Trait Implementations
//!
//! This module contains trait implementations for `SourceSpan`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SourceSpan;
use std::fmt;

impl fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(file) = &self.file {
            write!(f, "{}:{}..{}", file, self.start, self.end)
        } else {
            write!(f, "{}..{}", self.start, self.end)
        }
    }
}
