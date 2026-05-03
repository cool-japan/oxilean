//! # ScopeTable - Trait Implementations
//!
//! This module contains trait implementations for `ScopeTable`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ScopeTable;

impl<K: Eq, V: Clone> Default for ScopeTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
