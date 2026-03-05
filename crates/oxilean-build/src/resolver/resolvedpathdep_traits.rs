//! # ResolvedPathDep - Trait Implementations
//!
//! This module contains trait implementations for `ResolvedPathDep`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ResolvedPathDep;
use std::fmt;

impl fmt::Display for ResolvedPathDep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} @ {} ({})",
            self.name,
            self.path.display(),
            self.version
        )
    }
}
