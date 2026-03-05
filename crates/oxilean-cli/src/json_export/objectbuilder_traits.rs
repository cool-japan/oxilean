//! # ObjectBuilder - Trait Implementations
//!
//! This module contains trait implementations for `ObjectBuilder`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ObjectBuilder;
use std::fmt;

impl Default for ObjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}
