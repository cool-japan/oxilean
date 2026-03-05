//! # HoverProvider - Trait Implementations
//!
//! This module contains trait implementations for `HoverProvider`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HoverProvider;
use std::fmt;

impl Default for HoverProvider {
    fn default() -> Self {
        HoverProvider::new()
    }
}
