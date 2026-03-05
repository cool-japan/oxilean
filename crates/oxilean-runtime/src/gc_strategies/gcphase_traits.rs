//! # GcPhase - Trait Implementations
//!
//! This module contains trait implementations for `GcPhase`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GcPhase;
use std::fmt;

impl std::fmt::Display for GcPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
