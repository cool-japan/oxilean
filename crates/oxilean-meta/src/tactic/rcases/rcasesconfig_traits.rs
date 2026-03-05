//! # RcasesConfig - Trait Implementations
//!
//! This module contains trait implementations for `RcasesConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RcasesConfig;

impl Default for RcasesConfig {
    fn default() -> Self {
        Self {
            max_depth: 64,
            use_constructor_names: true,
            clear_unused: false,
        }
    }
}
