//! # GrothendieckGroup - Trait Implementations
//!
//! This module contains trait implementations for `GrothendieckGroup`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GrothendieckGroup;
use std::fmt;

impl std::fmt::Display for GrothendieckGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "K0({}) with {} generators, {} relations",
            self.monoid_name,
            self.generators.len(),
            self.relations.len()
        )
    }
}
