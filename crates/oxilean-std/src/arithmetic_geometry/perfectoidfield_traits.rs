//! # PerfectoidField - Trait Implementations
//!
//! This module contains trait implementations for `PerfectoidField`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PerfectoidField;
use std::fmt;

impl std::fmt::Display for PerfectoidField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (char 0, tilt: {})", self.name, self.tilt_name)
    }
}
