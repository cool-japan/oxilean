//! # DoBlockTracker - Trait Implementations
//!
//! This module contains trait implementations for `DoBlockTracker`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DoBlockTracker;

impl Default for DoBlockTracker {
    fn default() -> Self {
        Self::new(4)
    }
}
