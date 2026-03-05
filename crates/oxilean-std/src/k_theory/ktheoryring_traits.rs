//! # KTheoryRing - Trait Implementations
//!
//! This module contains trait implementations for `KTheoryRing`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::KTheoryRing;
use std::fmt;

impl std::fmt::Display for KTheoryRing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KTheory[{}]: K0 generators={}, K1 generators={}",
            self.name,
            self.k0_generators.len(),
            self.k1_generators.len()
        )
    }
}
