//! # ImplicitPipelineConfig - Trait Implementations
//!
//! This module contains trait implementations for `ImplicitPipelineConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ImplicitPipelineConfig;
use std::fmt;

impl Default for ImplicitPipelineConfig {
    fn default() -> Self {
        Self {
            max_implicit_args: 64,
            enable_tc_synthesis: true,
            allow_partial: false,
            insert_strict: false,
        }
    }
}
