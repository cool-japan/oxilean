//! # SimpReport - Trait Implementations
//!
//! This module contains trait implementations for `SimpReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::simp_types::SimpReport;

impl std::fmt::Display for SimpReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.proved {
            write!(f, "SimpReport {{ proved, {} }}", self.stats)
        } else if self.simplified {
            write!(f, "SimpReport {{ simplified, {} }}", self.stats)
        } else {
            write!(f, "SimpReport {{ unchanged, {} }}", self.stats)
        }
    }
}
