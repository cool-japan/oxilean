//! # Slot - Trait Implementations
//!
//! This module contains trait implementations for `Slot`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Slot;

impl<T> Default for Slot<T> {
    fn default() -> Self {
        Self::empty()
    }
}
