//! # DocumentationGenerator - Trait Implementations
//!
//! This module contains trait implementations for `DocumentationGenerator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DocumentationGenerator;
use std::fmt;

impl Default for DocumentationGenerator {
    fn default() -> Self {
        Self::new()
    }
}
