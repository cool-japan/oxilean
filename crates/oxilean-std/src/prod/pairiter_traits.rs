//! # PairIter - Trait Implementations
//!
//! This module contains trait implementations for `PairIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//! - `ExactSizeIterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PairIter;

impl<T: Clone> Iterator for PairIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match self.idx {
            0 => {
                self.idx = 1;
                Some(self.first.clone())
            }
            1 => {
                self.idx = 2;
                Some(self.second.clone())
            }
            _ => None,
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = 2usize.saturating_sub(self.idx);
        (remaining, Some(remaining))
    }
}

impl<T: Clone> ExactSizeIterator for PairIter<T> {}
