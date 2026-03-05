//! # StepKind - Trait Implementations
//!
//! This module contains trait implementations for `StepKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StepKind;
use std::fmt;

impl fmt::Display for StepKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse => write!(f, "parse"),
            Self::Elaborate => write!(f, "elaborate"),
            Self::Compile => write!(f, "compile"),
            Self::Link => write!(f, "link"),
            Self::Document => write!(f, "document"),
            Self::Test => write!(f, "test"),
            Self::Script(name) => write!(f, "script:{}", name),
            Self::CopyArtifact => write!(f, "copy-artifact"),
        }
    }
}
