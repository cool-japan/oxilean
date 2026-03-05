//! # ConfigBuilder - Trait Implementations
//!
//! This module contains trait implementations for `ConfigBuilder`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ConfigBuilder;
use std::fmt;

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
