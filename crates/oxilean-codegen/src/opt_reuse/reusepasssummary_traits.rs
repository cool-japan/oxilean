//! # ReusePassSummary - Trait Implementations
//!
//! This module contains trait implementations for `ReusePassSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReusePassSummary;
use std::fmt;

impl std::fmt::Display for ReusePassSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReusePassSummary[{}] {{ funcs={}, reuses={}, stack={}, saved={}B, {}us }}",
            self.pass_name,
            self.functions_analyzed,
            self.reuses_applied,
            self.stack_allocations,
            self.bytes_saved,
            self.duration_us,
        )
    }
}
