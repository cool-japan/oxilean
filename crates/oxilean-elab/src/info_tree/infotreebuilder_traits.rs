//! # InfoTreeBuilder - Trait Implementations
//!
//! This module contains trait implementations for `InfoTreeBuilder`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InfoTreeBuilder;
use std::fmt;

impl Default for InfoTreeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
