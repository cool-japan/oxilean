//! # DefEqConfig - Trait Implementations
//!
//! This module contains trait implementations for `DefEqConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::reduce::{Reducer, ReducibilityHint, TransparencyMode};

use super::types::DefEqConfig;

impl Default for DefEqConfig {
    fn default() -> Self {
        Self {
            max_steps: 1_000_000,
            proof_irrelevance: true,
            eta: true,
            lazy_delta: true,
            transparency: TransparencyMode::Reducible,
        }
    }
}
