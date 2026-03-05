//! # ParseErrorSimple - Trait Implementations
//!
//! This module contains trait implementations for `ParseErrorSimple`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseErrorSimple;

impl std::fmt::Display for ParseErrorSimple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error at {}: {}", self.pos, self.message)
    }
}
