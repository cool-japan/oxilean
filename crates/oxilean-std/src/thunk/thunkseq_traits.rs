//! # ThunkSeq - Trait Implementations
//!
//! This module contains trait implementations for `ThunkSeq`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ThunkSeq;

impl<T: Clone> Default for ThunkSeq<T> {
    fn default() -> Self {
        Self::new()
    }
}
