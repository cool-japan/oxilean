//! # HeapAllocSummary - Trait Implementations
//!
//! This module contains trait implementations for `HeapAllocSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HeapAllocSummary;
use std::fmt;

impl std::fmt::Display for HeapAllocSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HeapAlloc#{}({}, escape={}, unique={})",
            self.alloc_id, self.site, self.may_escape, self.is_unique
        )
    }
}
