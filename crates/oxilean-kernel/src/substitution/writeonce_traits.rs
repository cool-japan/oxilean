//! # WriteOnce - Trait Implementations
//!
//! This module contains trait implementations for `WriteOnce`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WriteOnce;

impl<T: Copy> Default for WriteOnce<T> {
    fn default() -> Self {
        Self::new()
    }
}
