//! # PrettyConfig - Trait Implementations
//!
//! This module contains trait implementations for `PrettyConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PrettyConfig;

impl Default for PrettyConfig {
    fn default() -> Self {
        PrettyConfig {
            indent: 2,
            max_width: 100,
            show_types: true,
            show_erased: false,
        }
    }
}
