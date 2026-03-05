//! # ImportConflict - Trait Implementations
//!
//! This module contains trait implementations for `ImportConflict`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ImportConflict;
use std::fmt;

impl fmt::Display for ImportConflict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name '{}' imported from both {} and {}",
            self.name, self.source1, self.source2
        )
    }
}
