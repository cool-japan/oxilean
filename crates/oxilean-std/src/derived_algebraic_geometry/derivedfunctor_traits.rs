//! # DerivedFunctor - Trait Implementations
//!
//! This module contains trait implementations for `DerivedFunctor`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DerivedFunctor;
use std::fmt;

impl fmt::Display for DerivedFunctor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = if self.is_left { "left" } else { "right" };
        write!(
            f,
            "{}: {} → {} ({})",
            self.name, self.source, self.target, side
        )
    }
}
