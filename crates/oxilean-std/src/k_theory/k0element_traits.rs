//! # K0Element - Trait Implementations
//!
//! This module contains trait implementations for `K0Element`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::K0Element;
use std::fmt;

impl std::fmt::Display for K0Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "K0[{}]: rank={} (pos={}, neg={})",
            self.label,
            self.virtual_rank(),
            self.pos_rank,
            self.neg_rank
        )
    }
}
