//! # ArcWeak - Trait Implementations
//!
//! This module contains trait implementations for `ArcWeak`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ArcWeak;
use std::fmt;

impl<T: fmt::Debug> fmt::Debug for ArcWeak<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArcWeak")
            .field(
                "alive",
                &self.alive.load(std::sync::atomic::Ordering::Acquire),
            )
            .finish()
    }
}
