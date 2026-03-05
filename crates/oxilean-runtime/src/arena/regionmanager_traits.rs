//! # RegionManager - Trait Implementations
//!
//! This module contains trait implementations for `RegionManager`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RegionManager;
use std::fmt;

impl Default for RegionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for RegionManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegionManager")
            .field("num_regions", &self.regions.len())
            .field("scope_depth", &self.scope_stack.len())
            .field("total_bytes_used", &self.total_bytes_used())
            .finish()
    }
}
