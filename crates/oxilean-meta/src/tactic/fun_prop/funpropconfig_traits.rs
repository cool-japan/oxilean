//! # FunPropConfig - Trait Implementations
//!
//! This module contains trait implementations for `FunPropConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FunPropConfig;

impl Default for FunPropConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            use_simp: true,
            verbose: false,
        }
    }
}
