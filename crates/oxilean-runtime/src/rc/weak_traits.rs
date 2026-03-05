//! # Weak - Trait Implementations
//!
//! This module contains trait implementations for `Weak`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Weak;
use std::fmt;

impl<T: fmt::Debug> fmt::Debug for Weak<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Weak")
            .field("alive", &self.alive.get())
            .finish()
    }
}
