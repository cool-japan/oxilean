//! # DependencyAnalyzer - Trait Implementations
//!
//! This module contains trait implementations for `DependencyAnalyzer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DependencyAnalyzer;
use std::fmt;

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
