//! # FutharkScanExpr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkScanExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkScanExpr;
use std::fmt;

impl std::fmt::Display for FutharkScanExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scan ({}) {} {}", self.op, self.neutral, self.array)
    }
}
