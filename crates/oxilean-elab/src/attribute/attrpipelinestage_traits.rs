//! # AttrPipelineStage - Trait Implementations
//!
//! This module contains trait implementations for `AttrPipelineStage`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AttrPipelineStage;
use std::fmt;

impl std::fmt::Display for AttrPipelineStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AttrPipelineStage::Parsed => "Parsed",
            AttrPipelineStage::Validated => "Validated",
            AttrPipelineStage::Applied => "Applied",
            AttrPipelineStage::PostProcessed => "PostProcessed",
            AttrPipelineStage::Done => "Done",
        };
        f.write_str(s)
    }
}
