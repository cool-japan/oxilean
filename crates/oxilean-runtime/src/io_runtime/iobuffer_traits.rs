//! # IoBuffer - Trait Implementations
//!
//! This module contains trait implementations for `IoBuffer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IoBuffer;

impl Default for IoBuffer {
    fn default() -> Self {
        Self::new(4096)
    }
}
