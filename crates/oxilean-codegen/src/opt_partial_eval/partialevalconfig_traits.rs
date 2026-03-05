//! # PartialEvalConfig - Trait Implementations
//!
//! This module contains trait implementations for `PartialEvalConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PartialEvalConfig;

impl Default for PartialEvalConfig {
    fn default() -> Self {
        PartialEvalConfig {
            max_specializations: 100,
            max_depth: 50,
            enable_memoization: true,
            specialize_hot_paths: true,
            aggressive_const_prop: false,
        }
    }
}
