//! # ModularTheoryData - Trait Implementations
//!
//! This module contains trait implementations for `ModularTheoryData`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ModularTheoryData;
use std::fmt;

impl std::fmt::Display for ModularTheoryData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ModularTheory[{}, type={}, Sd={:?}]",
            self.algebra_name, self.factor_type, self.sd_invariant
        )
    }
}
