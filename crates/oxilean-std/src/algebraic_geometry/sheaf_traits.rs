//! # Sheaf - Trait Implementations
//!
//! This module contains trait implementations for `Sheaf`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Sheaf;

impl<T: Clone> Default for Sheaf<T> {
    fn default() -> Self {
        Self::new()
    }
}
