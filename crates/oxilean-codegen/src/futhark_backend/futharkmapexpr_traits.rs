//! # FutharkMapExpr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkMapExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkMapExpr;
use std::fmt;

impl std::fmt::Display for FutharkMapExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrays = self.arrays.join(" ");
        write!(f, "map ({}) {}", self.func, arrays)
    }
}
