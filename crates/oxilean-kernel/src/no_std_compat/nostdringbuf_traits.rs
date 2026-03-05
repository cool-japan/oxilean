//! # NoStdRingBuf - Trait Implementations
//!
//! This module contains trait implementations for `NoStdRingBuf`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NoStdRingBuf;

impl<T> Default for NoStdRingBuf<T> {
    fn default() -> Self {
        Self::new()
    }
}
