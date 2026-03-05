//! # InferSessionConfig - Trait Implementations
//!
//! This module contains trait implementations for `InferSessionConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InferSessionConfig;
use std::fmt;

impl Default for InferSessionConfig {
    fn default() -> Self {
        InferSessionConfig::new()
    }
}
