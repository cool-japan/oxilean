//! # HoverFormatOptions - Trait Implementations
//!
//! This module contains trait implementations for `HoverFormatOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{HoverFormatOptions, TypeDisplayStyle};
use std::fmt;

impl Default for HoverFormatOptions {
    fn default() -> Self {
        Self {
            show_types: true,
            show_docs: true,
            show_source_link: true,
            show_examples: false,
            type_display_style: TypeDisplayStyle::Full,
            max_doc_lines: 10,
        }
    }
}
