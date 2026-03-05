//! # TraceDispatcher - Trait Implementations
//!
//! This module contains trait implementations for `TraceDispatcher`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TraceDispatcher;
use std::fmt;

impl Default for TraceDispatcher {
    fn default() -> Self {
        TraceDispatcher::new()
    }
}
