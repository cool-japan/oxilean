//! # UnivConstraint - Trait Implementations
//!
//! This module contains trait implementations for `UnivConstraint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::UnivConstraint;
use std::fmt;

impl fmt::Display for UnivConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnivConstraint::LeqLevel(a, b) => write!(f, "{:?} <= {:?}", a, b),
            UnivConstraint::EqLevel(a, b) => write!(f, "{:?} == {:?}", a, b),
            UnivConstraint::IsParam(l) => write!(f, "{:?} is param", l),
        }
    }
}
