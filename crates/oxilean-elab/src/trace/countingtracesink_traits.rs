//! # CountingTraceSink - Trait Implementations
//!
//! This module contains trait implementations for `CountingTraceSink`.
//!
//! ## Implemented Traits
//!
//! - `TraceSink`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TraceSink;
use super::types::{CountingTraceSink, TraceEvent};
use std::fmt;

impl TraceSink for CountingTraceSink {
    fn sink_name(&self) -> &'static str {
        "counting"
    }
    fn write_event(&mut self, event: &TraceEvent) {
        *self.counts.entry(event.level).or_insert(0) += 1;
    }
}
