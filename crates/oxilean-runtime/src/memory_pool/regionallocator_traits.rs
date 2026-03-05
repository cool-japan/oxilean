//! # RegionAllocator - Trait Implementations
//!
//! This module contains trait implementations for `RegionAllocator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RegionAllocator;

impl Default for RegionAllocator {
    fn default() -> Self {
        Self::new()
    }
}
