//! # ExpectedTypeStack - Trait Implementations
//!
//! This module contains trait implementations for `ExpectedTypeStack`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExpectedTypeStack;
use std::fmt;

impl Default for ExpectedTypeStack {
    fn default() -> Self {
        ExpectedTypeStack::new()
    }
}
