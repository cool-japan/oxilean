//! # AdditionalDerivableClass - Trait Implementations
//!
//! This module contains trait implementations for `AdditionalDerivableClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AdditionalDerivableClass;
use std::fmt;

impl fmt::Display for AdditionalDerivableClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}
