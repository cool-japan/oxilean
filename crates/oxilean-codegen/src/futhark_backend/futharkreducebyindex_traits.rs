//! # FutharkReduceByIndex - Trait Implementations
//!
//! This module contains trait implementations for `FutharkReduceByIndex`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkReduceByIndex;
use std::fmt;

impl std::fmt::Display for FutharkReduceByIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "reduce_by_index {} ({}) {} {} {}",
            self.dest, self.op, self.neutral, self.indices, self.values
        )
    }
}
