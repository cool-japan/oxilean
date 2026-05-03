//! # Reducibility - Trait Implementations
//!
//! This module contains trait implementations for `Reducibility`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Reducibility;
use std::fmt;

impl std::fmt::Display for Reducibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.attr_name())
    }
}
