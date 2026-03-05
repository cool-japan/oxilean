//! # TruncateStep - Trait Implementations
//!
//! This module contains trait implementations for `TruncateStep`.
//!
//! ## Implemented Traits
//!
//! - `TracePipelineStep`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TracePipelineStep;
use super::types::{TraceEvent, TruncateStep};
use std::fmt;

impl TracePipelineStep for TruncateStep {
    fn step_name(&self) -> &'static str {
        "truncate"
    }
    fn process(&self, mut events: Vec<TraceEvent>) -> Vec<TraceEvent> {
        events.truncate(self.max);
        events
    }
}
