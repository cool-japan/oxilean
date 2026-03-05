//! # ProjectiveSpace - Trait Implementations
//!
//! This module contains trait implementations for `ProjectiveSpace`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProjectiveSpace;
use std::fmt;

impl std::fmt::Display for ProjectiveSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P^{}", self.dim)
    }
}
