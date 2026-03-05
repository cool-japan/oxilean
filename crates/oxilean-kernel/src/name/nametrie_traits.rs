//! # NameTrie - Trait Implementations
//!
//! This module contains trait implementations for `NameTrie`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NameTrie;

impl<V> Default for NameTrie<V> {
    fn default() -> Self {
        Self {
            value: None,
            string_children: Vec::new(),
            num_children: Vec::new(),
        }
    }
}
