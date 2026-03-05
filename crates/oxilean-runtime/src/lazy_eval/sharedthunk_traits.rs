//! # SharedThunk - Trait Implementations
//!
//! This module contains trait implementations for `SharedThunk`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SharedThunk;
use std::fmt;

impl<T: Clone + fmt::Debug + Send + Sync + 'static> fmt::Debug for SharedThunk<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(v) = guard.value.get() {
            write!(f, "SharedThunk::Evaluated({:?})", v)
        } else {
            write!(f, "SharedThunk::Unevaluated")
        }
    }
}
