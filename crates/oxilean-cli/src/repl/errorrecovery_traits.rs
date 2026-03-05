//! # ErrorRecovery - Trait Implementations
//!
//! This module contains trait implementations for `ErrorRecovery`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ErrorRecovery;
use std::fmt;

impl Default for ErrorRecovery {
    fn default() -> Self {
        Self::new()
    }
}
