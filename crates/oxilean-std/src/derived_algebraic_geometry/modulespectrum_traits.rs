//! # ModuleSpectrum - Trait Implementations
//!
//! This module contains trait implementations for `ModuleSpectrum`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ModuleSpectrum;
use std::fmt;

impl fmt::Display for ModuleSpectrum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-Mod({})", self.base_ring, self.name)
    }
}
