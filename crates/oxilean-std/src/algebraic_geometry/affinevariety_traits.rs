//! # AffineVariety - Trait Implementations
//!
//! This module contains trait implementations for `AffineVariety`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AffineVariety;
use std::fmt;

impl std::fmt::Display for AffineVariety {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.equations.is_empty() {
            write!(f, "A^{}", self.ambient_dim)
        } else {
            write!(
                f,
                "V({}) in A^{}",
                self.equations.join(", "),
                self.ambient_dim
            )
        }
    }
}
