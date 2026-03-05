//! # StridePattern - Trait Implementations
//!
//! This module contains trait implementations for `StridePattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StridePattern;
use std::fmt;

impl std::fmt::Display for StridePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StridePattern::Unit => write!(f, "unit"),
            StridePattern::Constant(n) => write!(f, "const({})", n),
            StridePattern::Irregular => write!(f, "irregular"),
            StridePattern::Unknown => write!(f, "unknown"),
        }
    }
}
