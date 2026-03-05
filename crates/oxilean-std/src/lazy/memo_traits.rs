//! # Memo - Trait Implementations
//!
//! This module contains trait implementations for `Memo`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Memo;
use std::fmt;

impl<A: std::fmt::Debug + Send + Sync + 'static> std::fmt::Debug for Memo<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.lock.get() {
            Some(v) => write!(f, "Memo::Computed({:?})", v),
            None => write!(f, "Memo::Uninitialized"),
        }
    }
}
