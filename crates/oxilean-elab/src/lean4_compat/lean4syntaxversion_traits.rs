//! # Lean4SyntaxVersion - Trait Implementations
//!
//! This module contains trait implementations for `Lean4SyntaxVersion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Lean4SyntaxVersion;
use std::fmt;

impl std::fmt::Display for Lean4SyntaxVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
