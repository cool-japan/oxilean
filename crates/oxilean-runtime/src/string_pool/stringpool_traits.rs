//! # StringPool - Trait Implementations
//!
//! This module contains trait implementations for `StringPool`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::rope_fmt;
use super::types::StringPool;
use std::fmt;

impl Default for StringPool {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for StringPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StringPool({} strings)", self.strings.len())
    }
}
