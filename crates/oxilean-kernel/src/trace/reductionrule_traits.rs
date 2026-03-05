//! # ReductionRule - Trait Implementations
//!
//! This module contains trait implementations for `ReductionRule`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ReductionRule;
use std::fmt;

impl fmt::Display for ReductionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReductionRule::Beta => write!(f, "beta"),
            ReductionRule::Delta => write!(f, "delta"),
            ReductionRule::Iota => write!(f, "iota"),
            ReductionRule::Zeta => write!(f, "zeta"),
            ReductionRule::Eta => write!(f, "eta"),
            ReductionRule::Quot => write!(f, "quot"),
        }
    }
}
