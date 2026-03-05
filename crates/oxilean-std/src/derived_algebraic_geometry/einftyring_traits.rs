//! # EInftyRing - Trait Implementations
//!
//! This module contains trait implementations for `EInftyRing`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EInftyRing;
use std::fmt;

impl fmt::Display for EInftyRing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_sphere_spectrum {
            write!(f, "S (sphere spectrum, E∞)")
        } else {
            write!(f, "E∞-ring {}", self.name)
        }
    }
}
