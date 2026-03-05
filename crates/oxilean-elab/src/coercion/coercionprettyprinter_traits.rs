//! # CoercionPrettyPrinter - Trait Implementations
//!
//! This module contains trait implementations for `CoercionPrettyPrinter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoercionPrettyPrinter;
use std::fmt;

impl Default for CoercionPrettyPrinter {
    fn default() -> Self {
        CoercionPrettyPrinter::new()
    }
}
