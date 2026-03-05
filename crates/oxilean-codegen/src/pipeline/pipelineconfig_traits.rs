//! # PipelineConfig - Trait Implementations
//!
//! This module contains trait implementations for `PipelineConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::CodegenTarget;

use super::types::{OptLevel, PipelineConfig};

impl Default for PipelineConfig {
    fn default() -> Self {
        PipelineConfig {
            opt_level: OptLevel::O1,
            target: CodegenTarget::C,
            debug: false,
            emit_ir: false,
            passes: Vec::new(),
            max_iterations: 5,
            emit_comments: true,
        }
    }
}
