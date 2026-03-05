//! # MutualElabStage - Trait Implementations
//!
//! This module contains trait implementations for `MutualElabStage`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MutualElabStage;
use std::fmt;

impl std::fmt::Display for MutualElabStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            MutualElabStage::SigCollection => "SigCollection",
            MutualElabStage::DependencyAnalysis => "DependencyAnalysis",
            MutualElabStage::BodyElab => "BodyElab",
            MutualElabStage::TerminationCheck => "TerminationCheck",
            MutualElabStage::PostProcess => "PostProcess",
            MutualElabStage::Done => "Done",
        };
        f.write_str(name)
    }
}
