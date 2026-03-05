//! # Rope - Trait Implementations
//!
//! This module contains trait implementations for `Rope`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::rope_fmt;
use super::types::Rope;
use std::fmt;

impl Default for Rope {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(node) = &self.root {
            rope_fmt(node, f)
        } else {
            Ok(())
        }
    }
}
