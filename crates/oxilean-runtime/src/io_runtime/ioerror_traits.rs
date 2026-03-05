//! # IoError - Trait Implementations
//!
//! This module contains trait implementations for `IoError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IoError;
use std::fmt;

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref path) = self.path {
            write!(f, "{:?} error at {}: {}", self.kind, path, self.message)
        } else {
            write!(f, "{:?} error: {}", self.kind, self.message)
        }
    }
}

impl std::error::Error for IoError {}
