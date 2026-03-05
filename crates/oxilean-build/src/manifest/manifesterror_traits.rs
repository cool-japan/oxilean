//! # ManifestError - Trait Implementations
//!
//! This module contains trait implementations for `ManifestError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ManifestError;
use std::fmt;

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(msg) => write!(f, "invalid name: {}", msg),
            Self::InvalidVersion(msg) => write!(f, "invalid version: {}", msg),
            Self::UnknownFeature(msg) => write!(f, "unknown feature: {}", msg),
            Self::UnknownDependency(msg) => write!(f, "unknown dependency: {}", msg),
            Self::ParseError(msg) => write!(f, "parse error: {}", msg),
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::DuplicateKey(msg) => write!(f, "duplicate key: {}", msg),
            Self::MissingField(msg) => write!(f, "missing field: {}", msg),
        }
    }
}
