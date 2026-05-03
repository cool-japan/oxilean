//! # ArtifactKind - Trait Implementations
//!
//! This module contains trait implementations for `ArtifactKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ArtifactKind;
use std::fmt;

impl std::fmt::Display for ArtifactKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtifactKind::Object => write!(f, "object"),
            ArtifactKind::StaticLib => write!(f, "static-lib"),
            ArtifactKind::DynLib => write!(f, "dyn-lib"),
            ArtifactKind::Executable => write!(f, "executable"),
            ArtifactKind::Docs => write!(f, "docs"),
            ArtifactKind::Export => write!(f, "export"),
        }
    }
}
