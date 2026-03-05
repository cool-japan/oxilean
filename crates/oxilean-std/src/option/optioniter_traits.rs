//! # OptionIter - Trait Implementations
//!
//! This module contains trait implementations for `OptionIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OptionIter;

impl<T> Iterator for OptionIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.take()
    }
}
