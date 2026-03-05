//! # BuildTarget - Trait Implementations
//!
//! This module contains trait implementations for `BuildTarget`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildTarget;
use std::fmt;

impl fmt::Display for BuildTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildTarget::Check => write!(f, "check"),
            BuildTarget::Build => write!(f, "build"),
            BuildTarget::Release => write!(f, "release"),
            BuildTarget::Test => write!(f, "test"),
            BuildTarget::Bench => write!(f, "bench"),
            BuildTarget::Doc => write!(f, "doc"),
        }
    }
}
