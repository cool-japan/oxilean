//! # BuildStatus - Trait Implementations
//!
//! This module contains trait implementations for `BuildStatus`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildStatus;
use std::fmt;

impl fmt::Display for BuildStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildStatus::Success => write!(f, "OK"),
            BuildStatus::Failure => write!(f, "FAIL"),
            BuildStatus::Cached => write!(f, "CACHED"),
            BuildStatus::Skipped => write!(f, "SKIP"),
        }
    }
}
