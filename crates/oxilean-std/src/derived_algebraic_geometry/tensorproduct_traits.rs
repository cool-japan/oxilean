//! # TensorProduct - Trait Implementations
//!
//! This module contains trait implementations for `TensorProduct`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TensorProduct;
use std::fmt;

impl fmt::Display for TensorProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ⊗_{}^L {}", self.left, self.base, self.right)
    }
}
