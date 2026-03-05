//! # DependencyEdgeKind - Trait Implementations
//!
//! This module contains trait implementations for `DependencyEdgeKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DependencyEdgeKind;

impl std::fmt::Display for DependencyEdgeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyEdgeKind::Full => write!(f, "full"),
            DependencyEdgeKind::TypeOnly => write!(f, "type-only"),
            DependencyEdgeKind::Weak => write!(f, "weak"),
        }
    }
}
