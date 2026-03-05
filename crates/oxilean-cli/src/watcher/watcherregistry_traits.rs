//! # WatcherRegistry - Trait Implementations
//!
//! This module contains trait implementations for `WatcherRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WatcherRegistry;
use std::fmt;

impl Default for WatcherRegistry {
    fn default() -> Self {
        Self::new()
    }
}
