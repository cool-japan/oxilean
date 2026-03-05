//! # CtfeNumericRange - Trait Implementations
//!
//! This module contains trait implementations for `CtfeNumericRange`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeNumericRange;
use std::fmt;

impl std::fmt::Display for CtfeNumericRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.known_exact {
            write!(f, "{{{}}}", self.min)
        } else {
            write!(f, "[{}, {}]", self.min, self.max)
        }
    }
}
