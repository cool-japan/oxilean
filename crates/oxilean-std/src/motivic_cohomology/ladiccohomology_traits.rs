//! # LAdicCohomology - Trait Implementations
//!
//! This module contains trait implementations for `LAdicCohomology`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LAdicCohomology;
use std::fmt;

impl std::fmt::Display for LAdicCohomology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "H^{}_{{\u{00e9}t}}({}, ℤ_{})",
            self.degree, self.scheme, self.prime
        )
    }
}
