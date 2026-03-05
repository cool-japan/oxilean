//! # RecoveryStrategy - Trait Implementations
//!
//! This module contains trait implementations for `RecoveryStrategy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RecoveryStrategy;
use std::fmt;

impl fmt::Display for RecoveryStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecoveryStrategy::Abort => write!(f, "abort"),
            RecoveryStrategy::SkipToSync => write!(f, "skip-to-sync"),
            RecoveryStrategy::InsertToken => write!(f, "insert-token"),
            RecoveryStrategy::Replace => write!(f, "replace"),
        }
    }
}
