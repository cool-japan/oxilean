//! # ClosureConvertStats - Trait Implementations
//!
//! This module contains trait implementations for `ClosureConvertStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ClosureConvertStats;
use std::fmt;

impl fmt::Display for ClosureConvertStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ClosureConvertStats {{ converted={}, lifted={}, defunc={}, stack={}, heap={}, merged={} }}",
            self.closures_converted, self.helpers_lifted, self.defunctionalized, self
            .stack_allocated, self.heap_allocated, self.closures_merged,
        )
    }
}
