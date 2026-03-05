//! # LazyNormal - Trait Implementations
//!
//! This module contains trait implementations for `LazyNormal`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LazyNormal;

impl std::fmt::Debug for LazyNormal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_evaluated() {
            write!(
                f,
                "LazyNormal::Evaluated({:?})",
                self.normal
                    .get()
                    .expect("LazyNormal must be evaluated before Debug display")
            )
        } else {
            write!(f, "LazyNormal::Pending({:?})", self.original)
        }
    }
}
