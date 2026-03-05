//! # LazyRange - Trait Implementations
//!
//! This module contains trait implementations for `LazyRange`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LazyRange;

impl Iterator for LazyRange {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.step > 0 && self.current >= self.end {
            return None;
        }
        if self.step < 0 && self.current <= self.end {
            return None;
        }
        let val = self.current;
        self.current += self.step;
        Some(val)
    }
}
