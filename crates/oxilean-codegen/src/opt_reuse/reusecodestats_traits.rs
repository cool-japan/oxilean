//! # ReuseCodeStats - Trait Implementations
//!
//! This module contains trait implementations for `ReuseCodeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseCodeStats;
use std::fmt;

impl std::fmt::Display for ReuseCodeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReuseCodeStats {{ analyzed={}, reuses={}, stack={}, inlines={}, saved={}B }}",
            self.allocs_analyzed, self.reuses, self.stack_allocs, self.inlines, self.bytes_saved,
        )
    }
}
