//! # AtomicRefCounter - Trait Implementations
//!
//! This module contains trait implementations for `AtomicRefCounter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AtomicRefCounter;
use std::fmt;

impl Default for AtomicRefCounter {
    fn default() -> Self {
        Self::new(0)
    }
}

impl std::fmt::Display for AtomicRefCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AtomicRefCounter({})", self.load())
    }
}
