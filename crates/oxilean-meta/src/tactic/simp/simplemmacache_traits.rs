//! # SimpLemmaCache - Trait Implementations
//!
//! This module contains trait implementations for `SimpLemmaCache`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::simp_types::SimpLemmaCache;

impl std::fmt::Display for SimpLemmaCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SimpLemmaCache({} distinct lemmas, {} total)",
            self.lookups.len(),
            self.total_lookups()
        )
    }
}
