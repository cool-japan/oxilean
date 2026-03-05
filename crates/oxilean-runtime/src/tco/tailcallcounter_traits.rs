//! # TailCallCounter - Trait Implementations
//!
//! This module contains trait implementations for `TailCallCounter`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TailCallCounter;
use std::fmt;

impl fmt::Display for TailCallCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TailCallCounter {{ optimized: {}, max_depth: {} }}",
            self.optimized, self.max_depth
        )
    }
}
