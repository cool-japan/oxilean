//! # UnifyError - Trait Implementations
//!
//! This module contains trait implementations for `UnifyError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::UnifyError;
use std::fmt;

impl std::fmt::Display for UnifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnifyError::TypeMismatch(l, r) => {
                write!(f, "type mismatch: {:?} vs {:?}", l, r)
            }
            UnifyError::OccursCheck => write!(f, "occurs check failed"),
            UnifyError::LevelMismatch(l, r) => {
                write!(f, "level mismatch: {:?} vs {:?}", l, r)
            }
            UnifyError::Unsolvable(msg) => write!(f, "unsolvable: {}", msg),
            UnifyError::Other(msg) => write!(f, "unification error: {}", msg),
        }
    }
}
