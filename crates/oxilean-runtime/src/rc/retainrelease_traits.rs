//! # RetainRelease - Trait Implementations
//!
//! This module contains trait implementations for `RetainRelease`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RetainRelease;
use std::fmt;

impl<T: std::fmt::Debug> std::fmt::Debug for RetainRelease<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RetainRelease {{ live: {}, value: {:?} }}",
            self.live_count(),
            self.value
        )
    }
}
