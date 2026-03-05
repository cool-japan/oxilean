//! # NullTraceSink - Trait Implementations
//!
//! This module contains trait implementations for `NullTraceSink`.
//!
//! ## Implemented Traits
//!
//! - `TraceSink`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TraceSink;
use super::types::{NullTraceSink, TraceEvent};
use std::fmt;

impl TraceSink for NullTraceSink {
    fn sink_name(&self) -> &'static str {
        "null"
    }
    fn write_event(&mut self, _event: &TraceEvent) {}
}
