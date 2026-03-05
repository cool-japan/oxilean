//! # TakeIter - Trait Implementations
//!
//! This module contains trait implementations for `TakeIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TakeIter;
use std::fmt;

impl<T: Clone + fmt::Debug + 'static> Iterator for TakeIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let (head, tail_fn) = self.current.take()?;
        self.remaining -= 1;
        if let Some(f) = &tail_fn {
            let next_list = f();
            if let Some(h) = next_list.head {
                self.current = Some((h, next_list.tail));
            }
        }
        Some(head)
    }
}
