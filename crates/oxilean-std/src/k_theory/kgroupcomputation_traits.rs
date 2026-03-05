//! # KGroupComputation - Trait Implementations
//!
//! This module contains trait implementations for `KGroupComputation`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::KGroupComputation;
use std::fmt;

impl std::fmt::Display for KGroupComputation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe_k0())
    }
}
