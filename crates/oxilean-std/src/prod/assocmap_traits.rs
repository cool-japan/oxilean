//! # AssocMap - Trait Implementations
//!
//! This module contains trait implementations for `AssocMap`.
//!
//! ## Implemented Traits
//!
//! - `FromIterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AssocMap;

impl<K: PartialEq + Clone, V: Clone> FromIterator<(K, V)> for AssocMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = Self::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}
