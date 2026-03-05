//! # TwoPSet - Trait Implementations
//!
//! This module contains trait implementations for `TwoPSet`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TwoPSet;

impl<T: Clone + Eq + std::hash::Hash> Default for TwoPSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
