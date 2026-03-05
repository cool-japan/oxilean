//! # MacroTracer - Trait Implementations
//!
//! This module contains trait implementations for `MacroTracer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MacroTracer;
use std::fmt;

impl Default for MacroTracer {
    fn default() -> Self {
        MacroTracer::new(1000)
    }
}
