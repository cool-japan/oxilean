//! # UnifyConfig - Trait Implementations
//!
//! This module contains trait implementations for `UnifyConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::UnifyConfig;
use std::fmt;

impl Default for UnifyConfig {
    fn default() -> Self {
        Self {
            occurs_check: true,
            max_depth: 256,
            structural_sorts: false,
        }
    }
}
