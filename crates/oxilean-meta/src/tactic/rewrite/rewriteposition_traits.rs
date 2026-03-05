//! # RewritePosition - Trait Implementations
//!
//! This module contains trait implementations for `RewritePosition`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RewritePosition;

impl std::fmt::Display for RewritePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, step) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", step)?;
        }
        write!(f, "]")
    }
}
