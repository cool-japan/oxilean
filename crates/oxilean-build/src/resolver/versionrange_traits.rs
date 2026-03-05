//! # VersionRange - Trait Implementations
//!
//! This module contains trait implementations for `VersionRange`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VersionRange;
use std::fmt;

impl fmt::Display for VersionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.lower, &self.upper) {
            (None, None) => write!(f, "*"),
            (Some(lo), None) => write!(f, ">= {}", lo),
            (None, Some(hi)) => write!(f, "< {}", hi),
            (Some(lo), Some(hi)) => write!(f, ">= {}, < {}", lo, hi),
        }
    }
}
