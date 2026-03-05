//! # FormatCommandOptions - Trait Implementations
//!
//! This module contains trait implementations for `FormatCommandOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FormatCommandOptions;
use std::fmt;

impl Default for FormatCommandOptions {
    fn default() -> Self {
        Self {
            in_place: false,
            check: false,
            diff: false,
            recursive: true,
        }
    }
}
