//! # ParallelRegion - Trait Implementations
//!
//! This module contains trait implementations for `ParallelRegion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParallelRegion;
use std::fmt;

impl fmt::Display for ParallelRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ParallelRegion {{ func={}, kind={}, pattern={}, speedup={:.2}x, \
             shared={}, private={} }}",
            self.func_name,
            self.kind,
            self.pattern,
            self.estimated_speedup,
            self.shared_vars.len(),
            self.private_vars.len(),
        )
    }
}
