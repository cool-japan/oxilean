//! # CoqDiag - Trait Implementations
//!
//! This module contains trait implementations for `CoqDiag`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{CoqDiag, CoqDiagLevel};
use std::fmt;

impl std::fmt::Display for CoqDiag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = match self.level {
            CoqDiagLevel::Info => "info",
            CoqDiagLevel::Warning => "warning",
            CoqDiagLevel::Error => "error",
        };
        if let Some(item) = &self.item {
            write!(f, "[{}][{}] {}", l, item, self.message)
        } else {
            write!(f, "[{}] {}", l, self.message)
        }
    }
}
