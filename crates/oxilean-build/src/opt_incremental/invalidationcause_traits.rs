//! # InvalidationCause - Trait Implementations
//!
//! This module contains trait implementations for `InvalidationCause`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InvalidationCause;

impl std::fmt::Display for InvalidationCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidationCause::SourceChanged => write!(f, "source-changed"),
            InvalidationCause::DependencyInvalidated(dep) => {
                write!(f, "dependency-invalidated({})", dep)
            }
            InvalidationCause::Explicit => write!(f, "explicit"),
            InvalidationCause::CompilerVersionChanged => {
                write!(f, "compiler-version-changed")
            }
            InvalidationCause::BuildFlagChanged(flag) => {
                write!(f, "flag-changed({})", flag)
            }
        }
    }
}
