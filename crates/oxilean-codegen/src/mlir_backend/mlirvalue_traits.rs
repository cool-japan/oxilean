//! # MlirValue - Trait Implementations
//!
//! This module contains trait implementations for `MlirValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MlirValue;
use std::fmt;

impl fmt::Display for MlirValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.name)
    }
}
