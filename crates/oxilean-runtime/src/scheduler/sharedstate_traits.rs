//! # SharedState - Trait Implementations
//!
//! This module contains trait implementations for `SharedState`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use super::types::SharedState;

impl Default for SharedState {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for SharedState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedState")
            .field("shutdown", &self.should_shutdown())
            .field("task_counter", &self.task_counter.load(Ordering::Relaxed))
            .finish()
    }
}
