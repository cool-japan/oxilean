//! # FutharkIndexExpr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkIndexExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkIndexExpr;
use std::fmt;

impl std::fmt::Display for FutharkIndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.array, self.index)
    }
}
