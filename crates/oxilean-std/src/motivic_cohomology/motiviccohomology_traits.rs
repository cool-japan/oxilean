//! # MotivicCohomology - Trait Implementations
//!
//! This module contains trait implementations for `MotivicCohomology`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MotivicCohomology;
use std::fmt;

impl std::fmt::Display for MotivicCohomology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "H^{{{},{}}}({}, ℤ)",
            self.cohom_degree, self.weight, self.scheme
        )
    }
}
