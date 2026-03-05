//! # LibraryVersion - Trait Implementations
//!
//! This module contains trait implementations for `LibraryVersion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LibraryVersion;
use std::fmt;

impl fmt::Display for LibraryVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}.{}.{}",
            self.name, self.major, self.minor, self.patch
        )
    }
}
