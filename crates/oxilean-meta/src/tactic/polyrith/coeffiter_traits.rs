//! # CoeffIter - Trait Implementations
//!
//! This module contains trait implementations for `CoeffIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoeffIter;

impl<'a> Iterator for CoeffIter<'a> {
    type Item = Vec<i64>;
    fn next(&mut self) -> Option<Vec<i64>> {
        if self.done {
            return None;
        }
        let item: Vec<i64> = self.indices.iter().map(|&i| self.candidates[i]).collect();
        let n = self.indices.len();
        let base = self.candidates.len();
        let mut carry = true;
        for i in (0..n).rev() {
            if carry {
                self.indices[i] += 1;
                if self.indices[i] >= base {
                    self.indices[i] = 0;
                } else {
                    carry = false;
                }
            }
        }
        if carry {
            self.done = true;
        }
        Some(item)
    }
}
