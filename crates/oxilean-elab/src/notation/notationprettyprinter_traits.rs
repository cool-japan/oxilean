//! # NotationPrettyPrinter - Trait Implementations
//!
//! This module contains trait implementations for `NotationPrettyPrinter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NotationPrettyPrinter;
use std::fmt;

impl Default for NotationPrettyPrinter {
    fn default() -> Self {
        NotationPrettyPrinter::new()
    }
}
