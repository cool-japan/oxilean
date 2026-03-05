//! # FutharkSliceExpr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkSliceExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkSliceExpr;
use std::fmt;

impl std::fmt::Display for FutharkSliceExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.start.as_deref().unwrap_or("");
        let end = self.end.as_deref().unwrap_or("");
        if let Some(stride) = &self.stride {
            write!(f, "{}[{}:{}:{}]", self.array, start, end, stride)
        } else {
            write!(f, "{}[{}:{}]", self.array, start, end)
        }
    }
}
