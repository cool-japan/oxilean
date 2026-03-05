//! # BuildCycle - Trait Implementations
//!
//! This module contains trait implementations for `BuildCycle`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildCycle;

impl std::fmt::Display for BuildCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cycle[{}]", self.description())
    }
}
