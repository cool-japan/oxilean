//! # LexPair - Trait Implementations
//!
//! This module contains trait implementations for `LexPair`.
//!
//! ## Implemented Traits
//!
//! - `PartialOrd`
//! - `Ord`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LexPair;

impl<A: Ord, B: Ord> PartialOrd for LexPair<A, B> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<A: Ord, B: Ord> Ord for LexPair<A, B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.fst.cmp(&other.fst) {
            std::cmp::Ordering::Equal => self.snd.cmp(&other.snd),
            ord => ord,
        }
    }
}
