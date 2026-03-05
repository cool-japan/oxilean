//! # BiMap - Trait Implementations
//!
//! This module contains trait implementations for `BiMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BiMap;

impl<A, B> Default for BiMap<A, B>
where
    A: std::hash::Hash + Eq + Clone,
    B: std::hash::Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
