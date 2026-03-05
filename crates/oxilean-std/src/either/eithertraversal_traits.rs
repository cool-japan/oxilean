//! # EitherTraversal - Trait Implementations
//!
//! This module contains trait implementations for `EitherTraversal`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EitherTraversal;

#[allow(dead_code)]
impl<A, B> Default for EitherTraversal<A, B> {
    fn default() -> Self {
        Self::new()
    }
}
