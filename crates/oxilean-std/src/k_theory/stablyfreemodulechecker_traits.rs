//! # StablyFreeModuleChecker - Trait Implementations
//!
//! This module contains trait implementations for `StablyFreeModuleChecker`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StablyFreeModuleChecker;
use std::fmt;

impl std::fmt::Display for StablyFreeModuleChecker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.report())
    }
}
