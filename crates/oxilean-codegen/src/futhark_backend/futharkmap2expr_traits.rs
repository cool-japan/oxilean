//! # FutharkMap2Expr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkMap2Expr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkMap2Expr;
use std::fmt;

impl std::fmt::Display for FutharkMap2Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map2 ({}) {} {}", self.func, self.arr1, self.arr2)
    }
}
