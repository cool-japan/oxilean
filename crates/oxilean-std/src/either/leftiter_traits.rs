//! # LeftIter - Trait Implementations
//!
//! This module contains trait implementations for `LeftIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{LeftIter, OxiEither};

impl<A, B, I: Iterator<Item = OxiEither<A, B>>> Iterator for LeftIter<A, B, I> {
    type Item = A;
    fn next(&mut self) -> Option<A> {
        loop {
            match self.inner.next()? {
                OxiEither::Left(a) => return Some(a),
                OxiEither::Right(_) => continue,
            }
        }
    }
}
