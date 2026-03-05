//! # NotationConflictDetector - Trait Implementations
//!
//! This module contains trait implementations for `NotationConflictDetector`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NotationConflictDetector;
use std::fmt;

impl Default for NotationConflictDetector {
    fn default() -> Self {
        NotationConflictDetector::new()
    }
}
