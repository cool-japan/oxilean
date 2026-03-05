//! # FormatterConfig - Trait Implementations
//!
//! This module contains trait implementations for `FormatterConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FormatterConfig;
use std::fmt;

#[allow(dead_code)]
impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            line_width: 100,
            tab_size: 2,
            use_spaces: true,
            trailing_newline: true,
        }
    }
}
