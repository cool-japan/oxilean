//! # SortKind - Trait Implementations
//!
//! This module contains trait implementations for `SortKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SortKind;
use std::fmt;

impl fmt::Display for SortKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortKind::Type => write!(f, "Type"),
            SortKind::Prop => write!(f, "Prop"),
            SortKind::TypeU(u) => write!(f, "Type {}", u),
            SortKind::SortU(u) => write!(f, "Sort {}", u),
        }
    }
}
