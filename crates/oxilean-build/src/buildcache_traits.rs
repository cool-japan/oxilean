//! # BuildCache - Trait Implementations
//!
//! This module contains trait implementations for `BuildCache`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildCache;
use std::fmt;

impl std::fmt::Display for BuildCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuildCache({} entries)", self.entries.len())
    }
}
