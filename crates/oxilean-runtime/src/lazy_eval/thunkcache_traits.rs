//! # ThunkCache - Trait Implementations
//!
//! This module contains trait implementations for `ThunkCache`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ThunkCache;
use std::fmt;

impl Default for ThunkCache {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ThunkCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ThunkCache({})", self.entries.len())
    }
}
