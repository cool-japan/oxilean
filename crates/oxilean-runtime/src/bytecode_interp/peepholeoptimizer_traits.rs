//! # PeepholeOptimizer - Trait Implementations
//!
//! This module contains trait implementations for `PeepholeOptimizer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PeepholeOptimizer;

impl Default for PeepholeOptimizer {
    fn default() -> Self {
        Self::new(3)
    }
}
