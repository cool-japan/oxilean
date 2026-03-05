//! # AlgebraicKGroup - Trait Implementations
//!
//! This module contains trait implementations for `AlgebraicKGroup`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AlgebraicKGroup;
use std::fmt;

impl std::fmt::Display for AlgebraicKGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "K_{}({})", self.degree, self.ring)
    }
}
