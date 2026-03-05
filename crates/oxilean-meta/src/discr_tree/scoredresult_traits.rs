//! # ScoredResult - Trait Implementations
//!
//! This module contains trait implementations for `ScoredResult`.
//!
//! ## Implemented Traits
//!
//! - `PartialEq`
//! - `Eq`
//! - `PartialOrd`
//! - `Ord`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ScoredResult;

impl<T: Clone + PartialEq> PartialEq for ScoredResult<T> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.depth == other.depth
    }
}

impl<T: Clone + PartialEq> Eq for ScoredResult<T> {}

impl<T: Clone + PartialEq> PartialOrd for ScoredResult<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone + PartialEq> Ord for ScoredResult<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then(self.depth.cmp(&other.depth))
    }
}
