//! # SmtSort - Trait Implementations
//!
//! This module contains trait implementations for `SmtSort`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SmtSort;

impl std::fmt::Display for SmtSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmtSort::Bool => write!(f, "Bool"),
            SmtSort::Int => write!(f, "Int"),
            SmtSort::Real => write!(f, "Real"),
            SmtSort::BitVec(w) => write!(f, "(_ BitVec {})", w),
            SmtSort::Array(idx, elem) => write!(f, "(Array {} {})", idx, elem),
            SmtSort::Named(n) => write!(f, "{}", n),
        }
    }
}
