//! # FutharkReduceExpr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkReduceExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkReduceExpr;
use std::fmt;

impl std::fmt::Display for FutharkReduceExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "reduce ({}) {} {}", self.op, self.neutral, self.array)
    }
}
