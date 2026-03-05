//! # Morphism - Trait Implementations
//!
//! This module contains trait implementations for `Morphism`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Morphism;
use std::fmt;

impl std::fmt::Display for Morphism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} → {}", self.source, self.target)
    }
}
