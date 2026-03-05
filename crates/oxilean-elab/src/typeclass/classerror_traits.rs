//! # ClassError - Trait Implementations
//!
//! This module contains trait implementations for `ClassError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ClassError;
use std::fmt;

impl std::fmt::Display for ClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassError::NoInstance { class, ty } => {
                write!(f, "no instance for '{}' on type '{:?}'", class, ty)
            }
            ClassError::Ambiguous { class, candidates } => {
                write!(f, "ambiguous instances for '{}': {:?}", class, candidates)
            }
            ClassError::MissingMethod { instance, method } => {
                write!(f, "instance '{}' missing method '{}'", instance, method)
            }
            ClassError::Incoherent {
                class,
                inst_a,
                inst_b,
            } => {
                write!(
                    f,
                    "incoherent instances for '{}': '{}' and '{}'",
                    class, inst_a, inst_b
                )
            }
            ClassError::SuperclassFailed { class, superclass } => {
                write!(
                    f,
                    "superclass '{}' constraint failed for class '{}'",
                    superclass, class
                )
            }
        }
    }
}
