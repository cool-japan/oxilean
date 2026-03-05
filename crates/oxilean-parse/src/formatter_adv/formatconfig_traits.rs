//! # FormatConfig - Trait Implementations
//!
//! This module contains trait implementations for `FormatConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FormatConfig;

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            max_width: 100,
            indent_size: 2,
            use_unicode: false,
            spaces_around_ops: true,
            blank_between_decls: true,
            preserve_comments: true,
            normalize_whitespace: true,
            explicit_parens: false,
        }
    }
}
