//! # ReformatOptions - Trait Implementations
//!
//! This module contains trait implementations for `ReformatOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ReformatOptions;

impl Default for ReformatOptions {
    fn default() -> Self {
        Self {
            space_before_op: true,
            space_after_op: true,
            space_after_comma: true,
            no_space_before_close: true,
        }
    }
}
