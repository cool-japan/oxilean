//! # ParseWarning - Trait Implementations
//!
//! This module contains trait implementations for `ParseWarning`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseWarning;

impl std::fmt::Display for ParseWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: warning: {}", self.line, self.col, self.message)
    }
}
