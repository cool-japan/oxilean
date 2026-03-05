//! # AnnotationKind - Trait Implementations
//!
//! This module contains trait implementations for `AnnotationKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AnnotationKind;

impl std::fmt::Display for AnnotationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationKind::Info => write!(f, "info"),
            AnnotationKind::Deprecated => write!(f, "deprecated"),
            AnnotationKind::Suggestion => write!(f, "suggestion"),
        }
    }
}
