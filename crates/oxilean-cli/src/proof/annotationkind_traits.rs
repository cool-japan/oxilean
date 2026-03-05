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
use std::fmt;

impl fmt::Display for AnnotationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnnotationKind::Note => write!(f, "note"),
            AnnotationKind::Motivation => write!(f, "motivation"),
            AnnotationKind::Reference => write!(f, "reference"),
            AnnotationKind::Warning => write!(f, "warning"),
        }
    }
}
