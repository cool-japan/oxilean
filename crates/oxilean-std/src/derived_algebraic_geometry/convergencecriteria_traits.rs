//! # ConvergenceCriteria - Trait Implementations
//!
//! This module contains trait implementations for `ConvergenceCriteria`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ConvergenceCriteria;
use std::fmt;

impl fmt::Display for ConvergenceCriteria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvergenceCriteria::Weak => write!(f, "weakly converges"),
            ConvergenceCriteria::Strong => write!(f, "strongly converges"),
            ConvergenceCriteria::Conditional => write!(f, "conditionally converges"),
        }
    }
}
