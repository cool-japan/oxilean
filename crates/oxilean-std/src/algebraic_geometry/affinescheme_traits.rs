//! # AffineScheme - Trait Implementations
//!
//! This module contains trait implementations for `AffineScheme`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AffineScheme;
use std::fmt;

impl std::fmt::Display for AffineScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Spec({})", self.ring)
    }
}
