//! # LazyFilter - Trait Implementations
//!
//! This module contains trait implementations for `LazyFilter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LazyFilter;
use std::fmt;

impl<'a, A: Clone + fmt::Debug> Iterator for LazyFilter<'a, A> {
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.data.len() {
            let item = &self.data[self.index];
            self.index += 1;
            if (self.pred)(item) {
                return Some(item.clone());
            }
        }
        None
    }
}
