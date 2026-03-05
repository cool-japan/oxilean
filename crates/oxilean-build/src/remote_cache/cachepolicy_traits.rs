//! # CachePolicy - Trait Implementations
//!
//! This module contains trait implementations for `CachePolicy`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CachePolicy;

impl Default for CachePolicy {
    fn default() -> Self {
        Self::read_write()
    }
}
