//! # ChapelConfig - Trait Implementations
//!
//! This module contains trait implementations for `ChapelConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ChapelConfig;

impl Default for ChapelConfig {
    fn default() -> Self {
        ChapelConfig {
            indent_width: 2,
            annotate_vars: true,
            use_writeln: true,
        }
    }
}
