//! # SimpTrace - Trait Implementations
//!
//! This module contains trait implementations for `SimpTrace`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::simp_types::SimpTrace;

impl std::fmt::Display for SimpTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpTrace({} firings)", self.fired.len())
    }
}
