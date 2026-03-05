//! # TateModule - Trait Implementations
//!
//! This module contains trait implementations for `TateModule`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TateModule;
use std::fmt;

impl std::fmt::Display for TateModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "T_{}({})", self.prime, self.variety)
    }
}
