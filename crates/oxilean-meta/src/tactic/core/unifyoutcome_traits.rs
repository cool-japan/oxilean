//! # UnifyOutcome - Trait Implementations
//!
//! This module contains trait implementations for `UnifyOutcome`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::UnifyOutcome;

impl std::fmt::Display for UnifyOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnifyOutcome::Success => write!(f, "success"),
            UnifyOutcome::Failure(msg) => write!(f, "failure: {}", msg),
            UnifyOutcome::Deferred => write!(f, "deferred"),
        }
    }
}
