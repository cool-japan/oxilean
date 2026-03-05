//! # HypothesisIter - Trait Implementations
//!
//! This module contains trait implementations for `HypothesisIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{HypothesisIter, LocalEntry, LocalKind};
use std::fmt;

impl<'a> Iterator for HypothesisIter<'a> {
    type Item = &'a LocalEntry;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                None => return None,
                Some(e) if matches!(e.kind, LocalKind::Hypothesis) => return Some(e),
                Some(_) => continue,
            }
        }
    }
}
