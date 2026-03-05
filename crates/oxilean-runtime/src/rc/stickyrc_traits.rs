//! # StickyRc - Trait Implementations
//!
//! This module contains trait implementations for `StickyRc`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StickyRc;
use std::fmt;

impl Default for StickyRc {
    fn default() -> Self {
        Self::new(1, u32::MAX)
    }
}

impl std::fmt::Display for StickyRc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StickyRc({}/{})", self.count, self.max)
    }
}
