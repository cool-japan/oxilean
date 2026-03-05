//! # StyleConfig - Trait Implementations
//!
//! This module contains trait implementations for `StyleConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{AlignmentStyle, FunctionStyle, PatternStyle, StyleConfig};
use std::fmt;

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            function_style: FunctionStyle::Smart,
            pattern_style: PatternStyle::Compact,
            alignment: AlignmentStyle::Pipes,
        }
    }
}
