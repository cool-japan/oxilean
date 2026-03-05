//! # TabStopIterator - Trait Implementations
//!
//! This module contains trait implementations for `TabStopIterator`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TabStopIterator;

impl Iterator for TabStopIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current;
        self.current += self.tab_width;
        Some(val)
    }
}
