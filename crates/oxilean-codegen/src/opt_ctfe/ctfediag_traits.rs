//! # CtfeDiag - Trait Implementations
//!
//! This module contains trait implementations for `CtfeDiag`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{CtfeDiag, CtfeDiagLevel};
use std::fmt;

impl std::fmt::Display for CtfeDiag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = match self.level {
            CtfeDiagLevel::Debug => "debug",
            CtfeDiagLevel::Info => "info",
            CtfeDiagLevel::Warning => "warning",
            CtfeDiagLevel::Error => "error",
        };
        if let Some(func) = &self.func {
            write!(f, "[{}][{}] {}", level, func, self.message)
        } else {
            write!(f, "[{}] {}", level, self.message)
        }
    }
}
