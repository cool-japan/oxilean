//! # TanakaDuality - Trait Implementations
//!
//! This module contains trait implementations for `TanakaDuality`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TanakaDuality;
use std::fmt;

impl fmt::Display for TanakaDuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tanaka: {} ↔ {}  (equiv={})",
            self.lie_algebra,
            self.moduli_problem,
            self.verify_equivalence()
        )
    }
}
