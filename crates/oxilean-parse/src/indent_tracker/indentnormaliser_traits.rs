//! # IndentNormaliser - Trait Implementations
//!
//! This module contains trait implementations for `IndentNormaliser`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IndentNormaliser;

impl Default for IndentNormaliser {
    fn default() -> Self {
        Self::new(4)
    }
}
