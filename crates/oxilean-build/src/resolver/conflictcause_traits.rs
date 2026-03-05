//! # ConflictCause - Trait Implementations
//!
//! This module contains trait implementations for `ConflictCause`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ConflictCause;
use std::fmt;

impl fmt::Display for ConflictCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMatchingVersion {
                package,
                constraint,
            } => {
                write!(
                    f,
                    "no version of '{}' satisfies constraint '{}'",
                    package, constraint
                )
            }
            Self::IncompatibleRequirements {
                package,
                req_a,
                source_a,
                req_b,
                source_b,
            } => {
                write!(
                    f,
                    "incompatible requirements for '{}': {} (from {}) vs {} (from {})",
                    package, req_a, source_a, req_b, source_b
                )
            }
            Self::CyclicDependency { cycle } => {
                write!(f, "dependency cycle detected: {}", cycle.join(" -> "))
            }
            Self::MissingFeature { package, feature } => {
                write!(
                    f,
                    "feature '{}' not available in package '{}'",
                    feature, package
                )
            }
        }
    }
}
