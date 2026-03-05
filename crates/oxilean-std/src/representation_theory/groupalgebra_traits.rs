//! # GroupAlgebra - Trait Implementations
//!
//! This module contains trait implementations for `GroupAlgebra`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GroupAlgebra;
use std::fmt;

impl std::fmt::Display for GroupAlgebra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.field_char == 0 {
            write!(f, "\u{2102}[{}]", self.group_name)
        } else {
            write!(f, "\u{1D53D}_{}[{}]", self.field_char, self.group_name)
        }
    }
}
