//! # ErrorRenderer - Trait Implementations
//!
//! This module contains trait implementations for `ErrorRenderer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ErrorRenderer;

impl Default for ErrorRenderer {
    fn default() -> Self {
        Self::plain()
    }
}
