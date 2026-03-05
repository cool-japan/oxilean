//! # DerivableClass - Trait Implementations
//!
//! This module contains trait implementations for `DerivableClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DerivableClass;
use std::fmt;

impl fmt::Display for DerivableClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
