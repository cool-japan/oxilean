//! # ElabNote - Trait Implementations
//!
//! This module contains trait implementations for `ElabNote`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabNote;
use std::fmt;

impl std::fmt::Display for ElabNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.prefix(), self.message())
    }
}
