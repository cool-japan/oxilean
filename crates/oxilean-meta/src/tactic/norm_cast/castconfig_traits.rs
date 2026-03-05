//! # CastConfig - Trait Implementations
//!
//! This module contains trait implementations for `CastConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{MAX_CAST_CHAIN_DEPTH, MAX_CAST_STEPS};
use super::types::CastConfig;

impl Default for CastConfig {
    fn default() -> Self {
        Self {
            max_steps: MAX_CAST_STEPS,
            use_defaults: true,
            extra_lemmas: Vec::new(),
            simp_after: true,
            trace: false,
            max_chain_depth: MAX_CAST_CHAIN_DEPTH,
        }
    }
}
