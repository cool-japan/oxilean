//! # LinearConstraint - Trait Implementations
//!
//! This module contains trait implementations for `LinearConstraint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LinearConstraint;
use std::fmt;

impl fmt::Display for LinearConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinearConstraint::Le(e) => write!(f, "{e} ≤ 0"),
            LinearConstraint::Ge(e) => write!(f, "{e} ≥ 0"),
            LinearConstraint::Eq(e) => write!(f, "{e} = 0"),
        }
    }
}
