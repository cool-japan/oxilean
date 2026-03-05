//! # FutharkRotate - Trait Implementations
//!
//! This module contains trait implementations for `FutharkRotate`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkRotate;
use std::fmt;

impl std::fmt::Display for FutharkRotate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rotate @{} {} {}", self.dim, self.amount, self.array)
    }
}
