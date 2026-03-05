//! # NameSet - Trait Implementations
//!
//! This module contains trait implementations for `NameSet`.
//!
//! ## Implemented Traits
//!
//! - `FromIterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Name, NameSet};

impl FromIterator<Name> for NameSet {
    fn from_iter<I: IntoIterator<Item = Name>>(iter: I) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}
