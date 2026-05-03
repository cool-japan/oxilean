//! # BuildArtifact - Trait Implementations
//!
//! This module contains trait implementations for `BuildArtifact`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildArtifact;
use std::fmt;

impl std::fmt::Display for BuildArtifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}] -> {:?}", self.name, self.kind, self.path)
    }
}
