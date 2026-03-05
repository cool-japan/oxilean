//! # WatchHandle - Trait Implementations
//!
//! This module contains trait implementations for `WatchHandle`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WatchHandle;
use std::fmt;

impl Default for WatchHandle {
    fn default() -> Self {
        Self::new()
    }
}
