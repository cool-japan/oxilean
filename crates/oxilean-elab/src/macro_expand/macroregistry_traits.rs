//! # MacroRegistry - Trait Implementations
//!
//! This module contains trait implementations for `MacroRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MacroRegistry;
use std::fmt;

impl Default for MacroRegistry {
    fn default() -> Self {
        MacroRegistry::new()
    }
}
