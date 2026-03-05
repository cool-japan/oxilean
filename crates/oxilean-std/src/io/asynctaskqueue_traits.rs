//! # AsyncTaskQueue - Trait Implementations
//!
//! This module contains trait implementations for `AsyncTaskQueue`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AsyncTaskQueue;

impl Default for AsyncTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}
