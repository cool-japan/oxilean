//! # RcObserver - Trait Implementations
//!
//! This module contains trait implementations for `RcObserver`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RcObserver;

impl Default for RcObserver {
    fn default() -> Self {
        Self::new(1000)
    }
}
