//! # Completer - Trait Implementations
//!
//! This module contains trait implementations for `Completer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Completer;
use std::fmt;

impl Default for Completer {
    fn default() -> Self {
        Self::new()
    }
}
