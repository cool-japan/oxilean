//! # TraceAggregator - Trait Implementations
//!
//! This module contains trait implementations for `TraceAggregator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TraceAggregator;
use std::fmt;

impl Default for TraceAggregator {
    fn default() -> Self {
        TraceAggregator::new()
    }
}
