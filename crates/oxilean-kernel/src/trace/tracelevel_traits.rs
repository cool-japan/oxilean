//! # TraceLevel - Trait Implementations
//!
//! This module contains trait implementations for `TraceLevel`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TraceLevel;
use std::fmt;

impl fmt::Display for TraceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceLevel::Off => write!(f, "OFF"),
            TraceLevel::Error => write!(f, "ERROR"),
            TraceLevel::Warn => write!(f, "WARN"),
            TraceLevel::Info => write!(f, "INFO"),
            TraceLevel::Debug => write!(f, "DEBUG"),
            TraceLevel::Trace => write!(f, "TRACE"),
        }
    }
}
