//! # Tracer - Trait Implementations
//!
//! This module contains trait implementations for `Tracer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{TraceLevel, Tracer};
use std::fmt;

impl Default for Tracer {
    fn default() -> Self {
        Self::new(TraceLevel::Off)
    }
}
