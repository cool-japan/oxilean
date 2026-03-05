//! # NixValue - Trait Implementations
//!
//! This module contains trait implementations for `NixValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NixValue;
use std::fmt;

impl std::fmt::Display for NixValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_expr().emit(0))
    }
}
