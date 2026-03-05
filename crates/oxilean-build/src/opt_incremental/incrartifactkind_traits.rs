//! # IncrArtifactKind - Trait Implementations
//!
//! This module contains trait implementations for `IncrArtifactKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IncrArtifactKind;

impl std::fmt::Display for IncrArtifactKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncrArtifactKind::Object => write!(f, "object"),
            IncrArtifactKind::Interface => write!(f, "interface"),
            IncrArtifactKind::Docs => write!(f, "docs"),
            IncrArtifactKind::ProofExport => write!(f, "proof-export"),
        }
    }
}
