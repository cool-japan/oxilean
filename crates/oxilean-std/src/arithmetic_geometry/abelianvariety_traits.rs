//! # AbelianVariety - Trait Implementations
//!
//! This module contains trait implementations for `AbelianVariety`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AbelianVariety;
use std::fmt;

impl std::fmt::Display for AbelianVariety {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = if self.name.is_empty() {
            format!("A_{}", self.dimension)
        } else {
            self.name.clone()
        };
        write!(f, "{}/{} (dim {})", label, self.field, self.dimension)
    }
}
