//! # OwnershipLog - Trait Implementations
//!
//! This module contains trait implementations for `OwnershipLog`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OwnershipLog;

impl Default for OwnershipLog {
    fn default() -> Self {
        Self::new(1000)
    }
}
