//! # PrettyPrintOptions - Trait Implementations
//!
//! This module contains trait implementations for `PrettyPrintOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Language, PrettyPrintOptions};
use std::fmt;

#[allow(dead_code)]
impl Default for PrettyPrintOptions {
    fn default() -> Self {
        Self {
            max_width: 120,
            use_colour: false,
            show_suggestions: true,
            show_help: true,
            language: Language::English,
        }
    }
}
