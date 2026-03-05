//! # FutharkFilterExpr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkFilterExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkFilterExpr;
use std::fmt;

impl std::fmt::Display for FutharkFilterExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "filter ({}) {}", self.pred, self.array)
    }
}
