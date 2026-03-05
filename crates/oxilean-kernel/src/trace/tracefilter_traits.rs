//! # TraceFilter - Trait Implementations
//!
//! This module contains trait implementations for `TraceFilter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{TraceFilter, TraceLevel};
use std::fmt;

impl Default for TraceFilter {
    fn default() -> Self {
        Self::at_level(TraceLevel::Off)
    }
}
