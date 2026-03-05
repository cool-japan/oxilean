//! # MaySpectralSequence - Trait Implementations
//!
//! This module contains trait implementations for `MaySpectralSequence`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MaySpectralSequence;
use std::fmt;

impl fmt::Display for MaySpectralSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "May SS at p={}: {}", self.prime, self.e2_description())
    }
}
