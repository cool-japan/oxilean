//! # LetBindingIter - Trait Implementations
//!
//! This module contains trait implementations for `LetBindingIter`.
//!
//! ## Implemented Traits
//!
//! - `Iterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{LetBindingIter, LocalEntry, LocalKind};
use std::fmt;

impl<'a> Iterator for LetBindingIter<'a> {
    type Item = &'a LocalEntry;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                None => return None,
                Some(e) if matches!(e.kind, LocalKind::LetBinding) => return Some(e),
                Some(_) => continue,
            }
        }
    }
}
