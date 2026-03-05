//! # ThreadSafetyInfo - Trait Implementations
//!
//! This module contains trait implementations for `ThreadSafetyInfo`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ThreadSafetyInfo;
use std::fmt;

impl fmt::Display for ThreadSafetyInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ThreadSafetyInfo {{ safe={}, races={}, atomics_needed={} }}",
            self.is_thread_safe,
            self.race_conditions.len(),
            self.atomic_ops_needed.len()
        )
    }
}
