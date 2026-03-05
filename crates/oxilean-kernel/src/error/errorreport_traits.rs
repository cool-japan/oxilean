//! # ErrorReport - Trait Implementations
//!
//! This module contains trait implementations for `ErrorReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//! - `From`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{ErrorReport, KernelError};
use std::fmt;

impl fmt::Display for ErrorReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = &self.location {
            write!(f, "error in '{}': {}", loc, self.primary)?;
        } else {
            write!(f, "error: {}", self.primary)?;
        }
        for note in &self.notes {
            write!(f, "\n  note: {}", note)?;
        }
        Ok(())
    }
}

impl std::error::Error for ErrorReport {}

impl From<KernelError> for ErrorReport {
    fn from(err: KernelError) -> Self {
        ErrorReport::new(err)
    }
}
