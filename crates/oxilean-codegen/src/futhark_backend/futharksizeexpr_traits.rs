//! # FutharkSizeExpr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkSizeExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkSizeExpr;
use std::fmt;

impl std::fmt::Display for FutharkSizeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "length {}", self.array)
    }
}
