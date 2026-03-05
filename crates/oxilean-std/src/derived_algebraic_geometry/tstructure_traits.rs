//! # TStructure - Trait Implementations
//!
//! This module contains trait implementations for `TStructure`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TStructure;
use std::fmt;

impl fmt::Display for TStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "t-structure on {} — aisle: {}",
            self.category, self.aisle_description
        )
    }
}
