//! # ParallelKind - Trait Implementations
//!
//! This module contains trait implementations for `ParallelKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParallelKind;
use std::fmt;

impl fmt::Display for ParallelKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParallelKind::DataParallel => write!(f, "data-parallel"),
            ParallelKind::TaskParallel => write!(f, "task-parallel"),
            ParallelKind::PipelineParallel => write!(f, "pipeline-parallel"),
            ParallelKind::SpeculativeParallel => write!(f, "speculative-parallel"),
        }
    }
}
