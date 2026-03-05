//! # IoPolicy - Trait Implementations
//!
//! This module contains trait implementations for `IoPolicy`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IoPolicy;

impl Default for IoPolicy {
    fn default() -> Self {
        Self::strict()
    }
}
