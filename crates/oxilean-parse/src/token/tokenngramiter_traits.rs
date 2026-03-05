//! # TokenNgramIter - Trait Implementations
//!
//! This module contains trait implementations for `TokenNgramIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tokens::Token;

use super::types::TokenNgramIter;

impl<'a> Iterator for TokenNgramIter<'a> {
    type Item = &'a [Token];
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + self.window <= self.tokens.len() {
            let slice = &self.tokens[self.pos..self.pos + self.window];
            self.pos += 1;
            Some(slice)
        } else {
            None
        }
    }
}
