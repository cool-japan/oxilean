//! # PoolStatistics - Trait Implementations
//!
//! This module contains trait implementations for `PoolStatistics`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::rope_fmt;
use super::types::PoolStatistics;
use std::fmt;

impl fmt::Display for PoolStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PoolStatistics {{ unique: {}, total_requests: {}, bytes_saved: {} }}",
            self.unique_count,
            self.total_intern_requests,
            self.bytes_saved()
        )
    }
}
