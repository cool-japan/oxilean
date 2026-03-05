//! # Sigma - Trait Implementations
//!
//! This module contains trait implementations for `Sigma`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Sigma;
use std::fmt;

impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Sigma<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "⟨{}, {}⟩", self.fst, self.snd)
    }
}
