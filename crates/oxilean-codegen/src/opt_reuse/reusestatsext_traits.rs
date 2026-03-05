//! # ReuseStatsExt - Trait Implementations
//!
//! This module contains trait implementations for `ReuseStatsExt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseStatsExt;
use std::fmt;

impl std::fmt::Display for ReuseStatsExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReuseStatsExt {{ analyzed={}, reuses={}, stack={}, eliminated={}, bytes_saved={} }}",
            self.allocs_analyzed,
            self.reuses_applied,
            self.stack_allocations,
            self.allocs_eliminated,
            self.bytes_saved,
        )
    }
}
