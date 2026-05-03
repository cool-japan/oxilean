//! # BuildLogLevel - Trait Implementations
//!
//! This module contains trait implementations for `BuildLogLevel`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildLogLevel;
use std::fmt;

impl std::fmt::Display for BuildLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildLogLevel::Trace => write!(f, "TRACE"),
            BuildLogLevel::Info => write!(f, "INFO"),
            BuildLogLevel::Warn => write!(f, "WARN"),
            BuildLogLevel::Error => write!(f, "ERROR"),
        }
    }
}
