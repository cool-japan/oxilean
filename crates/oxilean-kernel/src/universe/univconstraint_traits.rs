//! # UnivConstraint - Trait Implementations
//!
//! This module contains trait implementations for `UnivConstraint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_level;
use super::types::UnivConstraint;

/// Display implementation for `UnivConstraint`.
impl std::fmt::Display for UnivConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnivConstraint::Lt(u, v) => {
                write!(f, "{} < {}", format_level(u), format_level(v))
            }
            UnivConstraint::Le(u, v) => {
                write!(f, "{} ≤ {}", format_level(u), format_level(v))
            }
            UnivConstraint::Eq(u, v) => {
                write!(f, "{} = {}", format_level(u), format_level(v))
            }
        }
    }
}
