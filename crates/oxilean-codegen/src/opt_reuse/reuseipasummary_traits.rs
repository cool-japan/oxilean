//! # ReuseIPASummary - Trait Implementations
//!
//! This module contains trait implementations for `ReuseIPASummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseIPASummary;
use std::fmt;

impl std::fmt::Display for ReuseIPASummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReuseIPASummary {{ regions={}, reuses={}, stack={}, saved={}B }}",
            self.regions.len(),
            self.global_reuse_count,
            self.global_stack_count,
            self.total_bytes_saved,
        )
    }
}
