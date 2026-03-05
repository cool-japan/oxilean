//! # PrintOptions - Trait Implementations
//!
//! This module contains trait implementations for `PrintOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PrintOptions;
use std::fmt;

impl Default for PrintOptions {
    fn default() -> Self {
        Self {
            max_width: 100,
            indent_size: 2,
            use_unicode: true,
        }
    }
}
