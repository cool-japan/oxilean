//! # FutharkPartitionExpr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkPartitionExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkPartitionExpr;
use std::fmt;

impl std::fmt::Display for FutharkPartitionExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "partition ({}) ({}) {}", self.k, self.pred, self.array)
    }
}
