//! # FormatOptions - Trait Implementations
//!
//! This module contains trait implementations for `FormatOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{FormatOptions, FormatRule};
use std::fmt;

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            indent_width: 2,
            max_width: 100,
            use_spaces: true,
            rules: vec![
                FormatRule::IndentBlock(2),
                FormatRule::BreakBeforeWhere,
                FormatRule::NormalizeOperators,
            ],
        }
    }
}
