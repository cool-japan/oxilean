//! # DpllSolver - Trait Implementations
//!
//! This module contains trait implementations for `DpllSolver`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DpllSolver;

impl Default for DpllSolver {
    fn default() -> Self {
        Self { next_var: 1 }
    }
}
