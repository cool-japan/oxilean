//! # HtmlConfig - Trait Implementations
//!
//! This module contains trait implementations for `HtmlConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HtmlConfig;
use std::fmt;

impl Default for HtmlConfig {
    fn default() -> Self {
        Self::new("OxiLean Documentation")
    }
}
