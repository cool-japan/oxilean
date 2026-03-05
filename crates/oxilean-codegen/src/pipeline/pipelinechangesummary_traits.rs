//! # PipelineChangeSummary - Trait Implementations
//!
//! This module contains trait implementations for `PipelineChangeSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::PipelineChangeSummary;
use std::fmt;

impl std::fmt::Display for PipelineChangeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PipelineChangeSummary {{ active: {:?}, converged: {:?} }}",
            self.active_passes, self.converged_passes
        )
    }
}
