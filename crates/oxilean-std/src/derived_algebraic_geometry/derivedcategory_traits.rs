//! # DerivedCategory - Trait Implementations
//!
//! This module contains trait implementations for `DerivedCategory`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DerivedCategory;
use std::fmt;

impl fmt::Display for DerivedCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bound = match (self.is_bounded_above, self.is_bounded_below) {
            (true, true) => "bounded",
            (true, false) => "bounded above",
            (false, true) => "bounded below",
            (false, false) => "unbounded",
        };
        write!(f, "{} ({})", self.name, bound)
    }
}
