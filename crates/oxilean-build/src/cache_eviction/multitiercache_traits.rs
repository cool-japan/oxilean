//! # MultiTierCache - Trait Implementations
//!
//! This module contains trait implementations for `MultiTierCache`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MultiTierCache;

impl std::fmt::Debug for MultiTierCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MultiTierCache")
            .field("l1_entries", &self.l1.len())
            .field("l2_entries", &self.l2.len())
            .field("l1_utilization", &self.l1.utilization())
            .field("l2_utilization", &self.l2.utilization())
            .finish()
    }
}
