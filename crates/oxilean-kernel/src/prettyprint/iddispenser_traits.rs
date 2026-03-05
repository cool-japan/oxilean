//! # IdDispenser - Trait Implementations
//!
//! This module contains trait implementations for `IdDispenser`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IdDispenser;

impl<T> Default for IdDispenser<T> {
    fn default() -> Self {
        Self::new()
    }
}
