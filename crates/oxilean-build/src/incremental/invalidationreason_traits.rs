//! # InvalidationReason - Trait Implementations
//!
//! This module contains trait implementations for `InvalidationReason`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InvalidationReason;
use std::fmt;

impl fmt::Display for InvalidationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceChanged { module, .. } => {
                write!(f, "source changed for module '{}'", module)
            }
            Self::DependencyInvalidated {
                module,
                changed_dep,
            } => {
                write!(
                    f,
                    "dependency '{}' invalidated module '{}'",
                    changed_dep, module
                )
            }
            Self::CompilerChanged {
                old_version,
                new_version,
            } => {
                write!(
                    f,
                    "compiler version changed from {} to {}",
                    old_version, new_version
                )
            }
            Self::FlagsChanged { module } => {
                write!(f, "build flags changed for module '{}'", module)
            }
            Self::Manual => write!(f, "manual invalidation"),
        }
    }
}
