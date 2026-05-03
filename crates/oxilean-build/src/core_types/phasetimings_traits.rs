//! # PhaseTimings - Trait Implementations
//!
//! This module contains trait implementations for `PhaseTimings`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PhaseTimings;
use std::fmt;

impl std::fmt::Display for PhaseTimings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}ms", self.phase, self.elapsed_ms)
    }
}
