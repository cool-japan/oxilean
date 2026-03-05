//! # FutharkDiag - Trait Implementations
//!
//! This module contains trait implementations for `FutharkDiag`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{FutharkDiag, FutharkDiagLevel};
use std::fmt;

impl std::fmt::Display for FutharkDiag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = match self.level {
            FutharkDiagLevel::Info => "info",
            FutharkDiagLevel::Warning => "warning",
            FutharkDiagLevel::Error => "error",
        };
        if let Some(loc) = &self.location {
            write!(f, "[{}] {}: {}", level, loc, self.message)
        } else {
            write!(f, "[{}] {}", level, self.message)
        }
    }
}
