//! # MatchExhaustExtMap - Trait Implementations
//!
//! This module contains trait implementations for `MatchExhaustExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MatchExhaustExtMap;

impl<V: Clone + Default> Default for MatchExhaustExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
