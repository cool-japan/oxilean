//! # LoopInfoSummary - Trait Implementations
//!
//! This module contains trait implementations for `LoopInfoSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LoopInfoSummary;
use std::fmt;

impl std::fmt::Display for LoopInfoSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoopInfoSummary {{ total={}, inner={}, countable={}, avg_depth={:.1}, max_depth={} }}",
            self.total_loops,
            self.inner_loops,
            self.countable_loops,
            self.avg_depth,
            self.max_depth,
        )
    }
}
