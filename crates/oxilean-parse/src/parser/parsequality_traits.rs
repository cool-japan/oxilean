//! # ParseQuality - Trait Implementations
//!
//! This module contains trait implementations for `ParseQuality`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseQuality;
use std::fmt;

impl fmt::Display for ParseQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseQuality::Failed => write!(f, "failed"),
            ParseQuality::Partial => write!(f, "partial"),
            ParseQuality::WithWarnings => write!(f, "with-warnings"),
            ParseQuality::Clean => write!(f, "clean"),
        }
    }
}
