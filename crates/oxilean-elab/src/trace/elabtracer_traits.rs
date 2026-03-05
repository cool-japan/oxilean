//! # ElabTracer - Trait Implementations
//!
//! This module contains trait implementations for `ElabTracer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{ElabTracer, TraceCategory, TraceLevel};
use std::fmt;

impl Default for ElabTracer {
    fn default() -> Self {
        ElabTracer {
            events: Vec::new(),
            level: TraceLevel::Off,
            enabled_categories: [
                TraceCategory::Elaboration,
                TraceCategory::TypeInference,
                TraceCategory::Unification,
                TraceCategory::InstanceSynthesis,
                TraceCategory::Coercion,
                TraceCategory::PatternMatch,
                TraceCategory::Tactic,
                TraceCategory::Notation,
                TraceCategory::Import,
            ]
            .iter()
            .copied()
            .collect(),
        }
    }
}
