//! # LevelFilterStep - Trait Implementations
//!
//! This module contains trait implementations for `LevelFilterStep`.
//!
//! ## Implemented Traits
//!
//! - `TracePipelineStep`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TracePipelineStep;
use super::types::{LevelFilterStep, TraceEvent};
use std::fmt;

impl TracePipelineStep for LevelFilterStep {
    fn step_name(&self) -> &'static str {
        "level_filter"
    }
    fn process(&self, events: Vec<TraceEvent>) -> Vec<TraceEvent> {
        // Keep events whose severity is at least min_level.
        // In the TraceLevel enum, lower variants = higher severity
        // (Error < Warn < Info < Debug < Trace), so we keep events
        // with e.level <= self.min_level.
        events
            .into_iter()
            .filter(|e| e.level <= self.min_level)
            .collect()
    }
}
