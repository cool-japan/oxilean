//! # FinIter - Trait Implementations
//!
//! This module contains trait implementations for `FinIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//! - `ExactSizeIterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Fin, FinIter};

impl Iterator for FinIter {
    type Item = Fin;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.bound {
            let f = Fin {
                val: self.current,
                bound: self.bound,
            };
            self.current += 1;
            Some(f)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.bound - self.current;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for FinIter {}
