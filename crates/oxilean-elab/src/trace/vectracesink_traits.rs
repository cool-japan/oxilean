//! # VecTraceSink - Trait Implementations
//!
//! This module contains trait implementations for `VecTraceSink`.
//!
//! ## Implemented Traits
//!
//! - `TraceSink`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TraceSink;
use super::types::{TraceEvent, VecTraceSink};
use std::fmt;

impl TraceSink for VecTraceSink {
    fn sink_name(&self) -> &'static str {
        "vec"
    }
    fn write_event(&mut self, event: &TraceEvent) {
        self.events.push(event.clone());
    }
}
