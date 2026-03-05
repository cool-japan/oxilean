//! # CraneliftValue - Trait Implementations
//!
//! This module contains trait implementations for `CraneliftValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CraneliftValue;
use std::fmt;

impl fmt::Display for CraneliftValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.id)
    }
}
