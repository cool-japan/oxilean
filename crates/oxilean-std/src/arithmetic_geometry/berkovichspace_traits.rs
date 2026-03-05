//! # BerkovichSpace - Trait Implementations
//!
//! This module contains trait implementations for `BerkovichSpace`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BerkovichSpace;
use std::fmt;

impl std::fmt::Display for BerkovichSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "M({}) over {}", self.algebra, self.base_field)
    }
}
