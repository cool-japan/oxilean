//! # DeadBindingConfig - Trait Implementations
//!
//! This module contains trait implementations for `DeadBindingConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::DeadBindingConfig;

impl Default for DeadBindingConfig {
    fn default() -> Self {
        DeadBindingConfig {
            remove_effectful: false,
            max_passes: 8,
        }
    }
}
