//! # ParseErrorGroup - Trait Implementations
//!
//! This module contains trait implementations for `ParseErrorGroup`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseErrorGroup;

impl std::fmt::Display for ParseErrorGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ParseErrorGroup[{}]({} errors)",
            self.label,
            self.errors.len()
        )
    }
}
