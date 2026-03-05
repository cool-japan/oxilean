//! # RcBitmask - Trait Implementations
//!
//! This module contains trait implementations for `RcBitmask`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RcBitmask;
use std::fmt;

impl Default for RcBitmask {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RcBitmask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RcBitmask({:#018x})", self.mask)
    }
}
