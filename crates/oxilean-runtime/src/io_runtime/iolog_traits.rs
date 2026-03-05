//! # IoLog - Trait Implementations
//!
//! This module contains trait implementations for `IoLog`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IoLog;

impl Default for IoLog {
    fn default() -> Self {
        Self::new(1000)
    }
}
