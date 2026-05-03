//! # BuildEnvironment - Trait Implementations
//!
//! This module contains trait implementations for `BuildEnvironment`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildEnvironment;
use std::fmt;

impl std::fmt::Display for BuildEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuildEnvironment({} vars)", self.vars.len())
    }
}
