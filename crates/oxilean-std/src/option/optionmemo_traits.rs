//! # OptionMemo - Trait Implementations
//!
//! This module contains trait implementations for `OptionMemo`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OptionMemo;

impl<K: std::hash::Hash + Eq, V: Clone> Default for OptionMemo<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
