//! # MemoryBudget - Trait Implementations
//!
//! This module contains trait implementations for `MemoryBudget`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MemoryBudget;
use std::fmt;

impl std::fmt::Display for MemoryBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MemoryBudget {{ used: {}/{}, peak: {} }}",
            self.used, self.limit, self.peak
        )
    }
}
