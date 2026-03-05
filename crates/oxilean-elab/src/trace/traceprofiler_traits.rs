//! # TraceProfiler - Trait Implementations
//!
//! This module contains trait implementations for `TraceProfiler`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TraceProfiler;
use std::fmt;

impl Default for TraceProfiler {
    fn default() -> Self {
        TraceProfiler::new()
    }
}
