//! # IndentFixer - Trait Implementations
//!
//! This module contains trait implementations for `IndentFixer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IndentFixer;

impl Default for IndentFixer {
    fn default() -> Self {
        Self::new(4, 4)
    }
}
