//! # DeduplicateStep - Trait Implementations
//!
//! This module contains trait implementations for `DeduplicateStep`.
//!
//! ## Implemented Traits
//!
//! - `TracePipelineStep`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use std::fmt;

use super::functions::TracePipelineStep;
use super::types::{DeduplicateStep, TraceEvent};

impl TracePipelineStep for DeduplicateStep {
    fn step_name(&self) -> &'static str {
        "deduplicate"
    }
    fn process(&self, events: Vec<TraceEvent>) -> Vec<TraceEvent> {
        let mut seen = HashSet::new();
        events
            .into_iter()
            .filter(|e| seen.insert(e.message.clone()))
            .collect()
    }
}
