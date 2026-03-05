//! # WorkStealingDeque - Trait Implementations
//!
//! This module contains trait implementations for `WorkStealingDeque`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WorkStealingDeque;
use std::fmt;

impl fmt::Debug for WorkStealingDeque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorkStealingDeque")
            .field("len", &self.deque.len())
            .field("capacity", &self.capacity)
            .finish()
    }
}
