//! # FloatInterval - Trait Implementations
//!
//! This module contains trait implementations for `FloatInterval`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FloatInterval;
use std::fmt;

impl std::fmt::Display for FloatInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.lo, self.hi)
    }
}
