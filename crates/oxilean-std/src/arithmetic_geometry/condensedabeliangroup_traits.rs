//! # CondensedAbelianGroup - Trait Implementations
//!
//! This module contains trait implementations for `CondensedAbelianGroup`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CondensedAbelianGroup;
use std::fmt;

impl std::fmt::Display for CondensedAbelianGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = if self.is_solid {
            "solid"
        } else if self.is_discrete {
            "discrete"
        } else {
            "condensed"
        };
        write!(f, "{}[{}]", kind, self.label)
    }
}
