//! # Formatter - Trait Implementations
//!
//! This module contains trait implementations for `Formatter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Formatter;
use std::fmt;

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}
