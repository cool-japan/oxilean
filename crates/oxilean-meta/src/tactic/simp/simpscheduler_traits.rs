//! # SimpScheduler - Trait Implementations
//!
//! This module contains trait implementations for `SimpScheduler`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::simp_types::SimpScheduler;

impl std::fmt::Display for SimpScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpScheduler({} lemmas)", self.len())
    }
}
