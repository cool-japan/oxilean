//! # ValidationWarning - Trait Implementations
//!
//! This module contains trait implementations for `ValidationWarning`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ValidationWarning;
use std::fmt;

impl std::fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationWarning::SorryProof(n) => write!(f, "theorem '{}' uses sorry", n),
            ValidationWarning::MissingAnnotation(n) => {
                write!(f, "declaration '{}' has no type annotation", n)
            }
            ValidationWarning::ManyConstructors(n, count) => {
                write!(
                    f,
                    "inductive '{}' has {} constructors (consider splitting)",
                    n, count
                )
            }
        }
    }
}
