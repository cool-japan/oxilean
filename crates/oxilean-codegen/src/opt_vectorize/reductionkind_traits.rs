//! # ReductionKind - Trait Implementations
//!
//! This module contains trait implementations for `ReductionKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ReductionKind;
use std::fmt;

impl fmt::Display for ReductionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReductionKind::Sum => write!(f, "sum"),
            ReductionKind::Product => write!(f, "product"),
            ReductionKind::Min => write!(f, "min"),
            ReductionKind::Max => write!(f, "max"),
            ReductionKind::And => write!(f, "and"),
            ReductionKind::Or => write!(f, "or"),
            ReductionKind::Xor => write!(f, "xor"),
            ReductionKind::DotProduct => write!(f, "dot_product"),
        }
    }
}
