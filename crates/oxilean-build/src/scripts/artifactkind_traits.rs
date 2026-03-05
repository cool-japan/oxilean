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

impl fmt::Display for ArtifactKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary => write!(f, "binary"),
            Self::StaticLib => write!(f, "static-lib"),
            Self::SharedLib => write!(f, "shared-lib"),
            Self::Module => write!(f, "module"),
            Self::Header => write!(f, "header"),
            Self::Doc => write!(f, "doc"),
            Self::Data => write!(f, "data"),
        }
    }
}
