//! # LazyMap2 - Trait Implementations
//!
//! This module contains trait implementations for `LazyMap2`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LazyMap2;
use std::fmt;

impl<'a, A, B: fmt::Debug> Iterator for LazyMap2<'a, A, B> {
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.data.get(self.index).map(|x| (self.func)(x));
        if result.is_some() {
            self.index += 1;
        }
        result
    }
}
