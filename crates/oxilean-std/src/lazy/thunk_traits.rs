//! # Thunk - Trait Implementations
//!
//! This module contains trait implementations for `Thunk`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Deferred, Thunk};
use std::fmt;

impl<A: std::fmt::Debug> std::fmt::Debug for Thunk<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.cell.get() {
            Some(v) => write!(f, "Thunk::Evaluated({:?})", v),
            None => write!(f, "Thunk::Deferred"),
        }
    }
}
