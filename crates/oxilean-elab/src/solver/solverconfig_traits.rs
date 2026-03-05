//! # SolverConfig - Trait Implementations
//!
//! This module contains trait implementations for `SolverConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SolverConfig;
use std::fmt;

impl Default for SolverConfig {
    fn default() -> Self {
        SolverConfig {
            max_iterations: 1000,
            occurs_check: true,
            enable_postponing: true,
            max_postponed: 200,
            log_events: false,
            strict_mode: false,
        }
    }
}
