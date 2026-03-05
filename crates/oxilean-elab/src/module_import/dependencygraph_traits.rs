//! # DependencyGraph - Trait Implementations
//!
//! This module contains trait implementations for `DependencyGraph`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DependencyGraph;
use std::fmt;

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}
