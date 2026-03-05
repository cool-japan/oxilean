//! # RightIter - Trait Implementations
//!
//! This module contains trait implementations for `RightIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{OxiEither, RightIter};

impl<A, B, I: Iterator<Item = OxiEither<A, B>>> Iterator for RightIter<A, B, I> {
    type Item = B;
    fn next(&mut self) -> Option<B> {
        loop {
            match self.inner.next()? {
                OxiEither::Left(_) => continue,
                OxiEither::Right(b) => return Some(b),
            }
        }
    }
}
