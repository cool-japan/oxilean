//! # StrengthReductionPass - Trait Implementations
//!
//! This module contains trait implementations for `StrengthReductionPass`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StrengthReductionPass;
use std::fmt;

impl Default for StrengthReductionPass {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for StrengthReductionPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StrengthReductionPass(reductions={})", self.reductions)
    }
}
