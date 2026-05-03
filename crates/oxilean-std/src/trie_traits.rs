//! # Trie - Trait Implementations
//!
//! This module contains trait implementations for `Trie`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Trie;

impl<V> Default for Trie<V> {
    fn default() -> Self {
        Self::new()
    }
}
