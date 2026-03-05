//! # DeformationFunctor - Trait Implementations
//!
//! This module contains trait implementations for `DeformationFunctor`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DeformationFunctor;
use std::fmt;

impl fmt::Display for DeformationFunctor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Def_{}  (controlled by {})",
            self.space, self.controlling_lie
        )
    }
}
