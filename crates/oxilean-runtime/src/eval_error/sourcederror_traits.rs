//! # SourcedError - Trait Implementations
//!
//! This module contains trait implementations for `SourcedError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SourcedError;
use std::fmt;

impl fmt::Display for SourcedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.source, self.error)
    }
}
