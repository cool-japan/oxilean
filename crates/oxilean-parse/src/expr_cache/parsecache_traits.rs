//! # ParseCache - Trait Implementations
//!
//! This module contains trait implementations for `ParseCache`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseCache;

impl Default for ParseCache {
    fn default() -> Self {
        Self::new(1024)
    }
}
