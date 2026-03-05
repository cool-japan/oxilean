//! # NameMap - Trait Implementations
//!
//! This module contains trait implementations for `NameMap`.
//!
//! ## Implemented Traits
//!
//! - `FromIterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Name, NameMap};

impl<V> FromIterator<(Name, V)> for NameMap<V> {
    fn from_iter<I: IntoIterator<Item = (Name, V)>>(iter: I) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}
