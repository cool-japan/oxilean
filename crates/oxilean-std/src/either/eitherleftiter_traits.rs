//! # EitherLeftIter - Trait Implementations
//!
//! This module contains trait implementations for `EitherLeftIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{EitherLeftIter, OxiEither};

impl<A: Clone, B> Iterator for EitherLeftIter<A, B> {
    type Item = A;
    fn next(&mut self) -> Option<A> {
        match self.inner.take() {
            Some(OxiEither::Left(a)) => Some(a),
            _ => None,
        }
    }
}
