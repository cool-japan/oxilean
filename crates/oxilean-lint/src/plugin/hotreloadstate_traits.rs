//! # HotReloadState - Trait Implementations
//!
//! This module contains trait implementations for `HotReloadState`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HotReloadState;

impl Default for HotReloadState {
    fn default() -> Self {
        Self::new(false)
    }
}
