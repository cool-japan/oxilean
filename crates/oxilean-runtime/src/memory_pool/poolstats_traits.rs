//! # PoolStats - Trait Implementations
//!
//! This module contains trait implementations for `PoolStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PoolStats;
use std::fmt;

impl std::fmt::Display for PoolStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PoolStats {{ allocated: {}, free: {}, total: {}, slabs: {} }}",
            self.allocated, self.free, self.total, self.slab_count
        )
    }
}
