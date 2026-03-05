//! # ResourceConstraints - Trait Implementations
//!
//! This module contains trait implementations for `ResourceConstraints`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ResourceConstraints;

impl Default for ResourceConstraints {
    fn default() -> Self {
        Self::new(1)
    }
}

