//! # PapQueue - Trait Implementations
//!
//! This module contains trait implementations for `PapQueue`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PapQueue;

impl Default for PapQueue {
    fn default() -> Self {
        Self::new(256)
    }
}
