//! # PrettyConfig - Trait Implementations
//!
//! This module contains trait implementations for `PrettyConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{ParensMode, PrettyConfig};

impl Default for PrettyConfig {
    fn default() -> Self {
        Self {
            max_width: 100,
            indent_size: 2,
            show_implicit: false,
            show_universes: false,
            use_unicode: false,
            use_notation: true,
            parens_mode: ParensMode::Minimal,
        }
    }
}
