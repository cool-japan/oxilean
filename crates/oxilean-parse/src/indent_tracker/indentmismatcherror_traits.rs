//! # IndentMismatchError - Trait Implementations
//!
//! This module contains trait implementations for `IndentMismatchError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IndentMismatchError;

impl std::fmt::Display for IndentMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message())
    }
}
