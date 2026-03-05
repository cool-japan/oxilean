//! # InternedString - Trait Implementations
//!
//! This module contains trait implementations for `InternedString`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::rope_fmt;
use super::types::InternedString;
use std::fmt;

impl fmt::Debug for InternedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InternedString({})", self.index)
    }
}

impl fmt::Display for InternedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.index)
    }
}
