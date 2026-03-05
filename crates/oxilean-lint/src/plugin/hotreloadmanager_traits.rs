//! # HotReloadManager - Trait Implementations
//!
//! This module contains trait implementations for `HotReloadManager`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HotReloadManager;

impl Default for HotReloadManager {
    fn default() -> Self {
        Self::new(false)
    }
}
