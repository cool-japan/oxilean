//! # DefEqConfig - Trait Implementations
//!
//! This module contains trait implementations for `DefEqConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DefEqConfig;

impl Default for DefEqConfig {
    fn default() -> Self {
        Self {
            proof_irrelevance: true,
            eta_reduction: true,
            lazy_delta: true,
            max_delta_steps: 1024,
        }
    }
}
