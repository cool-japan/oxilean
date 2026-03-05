//! # IdxRange - Trait Implementations
//!
//! This module contains trait implementations for `IdxRange`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IdxRange;

impl<T> std::fmt::Debug for IdxRange<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IdxRange({}..{})", self.start.raw, self.end.raw)
    }
}
