//! # ErrorReport - Trait Implementations
//!
//! This module contains trait implementations for `ErrorReport`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ErrorReport;
use std::fmt;

impl Default for ErrorReport {
    fn default() -> Self {
        Self::new()
    }
}
