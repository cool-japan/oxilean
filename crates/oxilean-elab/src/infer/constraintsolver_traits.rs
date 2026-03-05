//! # ConstraintSolver - Trait Implementations
//!
//! This module contains trait implementations for `ConstraintSolver`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ConstraintSolver;
use std::fmt;

impl Default for ConstraintSolver {
    fn default() -> Self {
        Self::new()
    }
}
