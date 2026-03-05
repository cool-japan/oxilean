//! # CountingStep - Trait Implementations
//!
//! This module contains trait implementations for `CountingStep`.
//!
//! ## Implemented Traits
//!
//! - `ProfilingStep`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::ProfilingStep;
use super::types::{CountingStep, ProfilingEvent};

impl ProfilingStep for CountingStep {
    fn process(&mut self, events: &[(u64, ProfilingEvent)]) {
        for (_, event) in events {
            *self
                .counts
                .entry(Self::variant_name(event).to_string())
                .or_insert(0) += 1;
        }
    }
    fn name(&self) -> &str {
        &self.step_name
    }
}
