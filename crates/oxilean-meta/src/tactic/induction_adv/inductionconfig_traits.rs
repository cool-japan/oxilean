//! # InductionConfig - Trait Implementations
//!
//! This module contains trait implementations for `InductionConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InductionConfig;

impl Default for InductionConfig {
    fn default() -> Self {
        Self {
            generalizing: Vec::new(),
            using_recursor: None,
            with_names: Vec::new(),
            revert_deps: true,
            clear_target: true,
            simp_lemmas: Vec::new(),
            max_depth: 128,
        }
    }
}
