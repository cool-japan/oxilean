//! # StdVersion - Trait Implementations
//!
//! This module contains trait implementations for `StdVersion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StdVersion;

impl std::fmt::Display for StdVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pre.is_empty() {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        } else {
            write!(
                f,
                "{}.{}.{}-{}",
                self.major, self.minor, self.patch, self.pre
            )
        }
    }
}
