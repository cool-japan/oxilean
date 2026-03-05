//! # ProofSkeleton - Trait Implementations
//!
//! This module contains trait implementations for `ProofSkeleton`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProofSkeleton;

impl std::fmt::Display for ProofSkeleton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProofSkeleton({} holes)", self.holes.len())
    }
}
