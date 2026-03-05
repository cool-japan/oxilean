//! # Thunk - Trait Implementations
//!
//! This module contains trait implementations for `Thunk`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Thunk;
use std::fmt;

impl<T: Clone + fmt::Debug> fmt::Debug for Thunk<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Thunk({:?})", *self.state.borrow())
    }
}
