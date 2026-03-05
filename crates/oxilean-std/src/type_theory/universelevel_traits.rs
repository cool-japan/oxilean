//! # UniverseLevel - Trait Implementations
//!
//! This module contains trait implementations for `UniverseLevel`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::UniverseLevel;
use std::fmt;

impl std::fmt::Display for UniverseLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type_{}", self.0)
    }
}
