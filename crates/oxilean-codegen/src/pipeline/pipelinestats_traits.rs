//! # PipelineStats - Trait Implementations
//!
//! This module contains trait implementations for `PipelineStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::PipelineStats;
use std::fmt;

impl fmt::Display for PipelineStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PipelineStats {{")?;
        writeln!(
            f,
            "  total_time={}us, iterations={}, decls: {} -> {}",
            self.total_time_us, self.iterations, self.input_decls, self.output_decls,
        )?;
        for (pass_id, stats) in &self.per_pass {
            writeln!(f, "  {}: {}", pass_id, stats)?;
        }
        write!(f, "}}")
    }
}
