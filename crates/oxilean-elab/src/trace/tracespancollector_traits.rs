//! # TraceSpanCollector - Trait Implementations
//!
//! This module contains trait implementations for `TraceSpanCollector`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TraceSpanCollector;
use std::fmt;

impl Default for TraceSpanCollector {
    fn default() -> Self {
        TraceSpanCollector::new()
    }
}
