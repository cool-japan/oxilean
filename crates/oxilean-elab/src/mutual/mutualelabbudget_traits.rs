//! # MutualElabBudget - Trait Implementations
//!
//! This module contains trait implementations for `MutualElabBudget`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MutualElabBudget;
use std::fmt;

impl Default for MutualElabBudget {
    fn default() -> Self {
        Self {
            max_scc_size: 32,
            max_termination_depth: 128,
            max_structural_args: 16,
            max_refinements: 8,
        }
    }
}
