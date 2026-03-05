//! # Fin - Trait Implementations
//!
//! This module contains trait implementations for `Fin`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Fin;
use std::fmt;

impl std::fmt::Display for Fin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fin({}, {})", self.val, self.bound)
    }
}
