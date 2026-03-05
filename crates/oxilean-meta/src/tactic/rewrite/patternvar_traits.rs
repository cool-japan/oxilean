//! # PatternVar - Trait Implementations
//!
//! This module contains trait implementations for `PatternVar`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PatternVar;

impl std::fmt::Display for PatternVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.0)
    }
}
