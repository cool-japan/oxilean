//! # AnnotatedError - Trait Implementations
//!
//! This module contains trait implementations for `AnnotatedError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AnnotatedError;
use std::fmt;

impl fmt::Display for AnnotatedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;
        for note in &self.notes {
            write!(f, "\n  {}", note)?;
        }
        Ok(())
    }
}

impl std::error::Error for AnnotatedError {}
