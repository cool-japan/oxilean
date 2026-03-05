//! # Unifier - Trait Implementations
//!
//! This module contains trait implementations for `Unifier`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Unifier;
use std::fmt;

impl Default for Unifier {
    fn default() -> Self {
        Self::new()
    }
}
