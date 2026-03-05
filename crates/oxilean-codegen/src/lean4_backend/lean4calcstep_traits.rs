//! # Lean4CalcStep - Trait Implementations
//!
//! This module contains trait implementations for `Lean4CalcStep`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Lean4CalcStep;
use std::fmt;

impl fmt::Display for Lean4CalcStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} := {}",
            self.lhs, self.relation, self.rhs, self.justification
        )
    }
}
