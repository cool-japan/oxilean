//! # IncrementalError - Trait Implementations
//!
//! This module contains trait implementations for `IncrementalError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IncrementalError;
use std::fmt;

impl fmt::Display for IncrementalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CyclicModuleDependency => {
                write!(f, "cyclic module dependency detected")
            }
            Self::ModuleNotFound(m) => write!(f, "module not found: {}", m),
            Self::CorruptedArtifact(m) => write!(f, "corrupted artifact: {}", m),
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::CacheError(e) => write!(f, "cache error: {}", e),
        }
    }
}
