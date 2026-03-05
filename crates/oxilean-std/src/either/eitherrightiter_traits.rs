//! # EitherRightIter - Trait Implementations
//!
//! This module contains trait implementations for `EitherRightIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{EitherRightIter, OxiEither};

impl<A, B: Clone> Iterator for EitherRightIter<A, B> {
    type Item = B;
    fn next(&mut self) -> Option<B> {
        match self.inner.take() {
            Some(OxiEither::Right(b)) => Some(b),
            _ => None,
        }
    }
}
