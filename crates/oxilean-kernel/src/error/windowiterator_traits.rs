//! # WindowIterator - Trait Implementations
//!
//! This module contains trait implementations for `WindowIterator`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WindowIterator;
use std::fmt;

impl<'a, T> Iterator for WindowIterator<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + self.window > self.data.len() {
            return None;
        }
        let slice = &self.data[self.pos..self.pos + self.window];
        self.pos += 1;
        Some(slice)
    }
}
