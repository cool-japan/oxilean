//! # MacroEnvironment - Trait Implementations
//!
//! This module contains trait implementations for `MacroEnvironment`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MacroEnvironment;
use std::fmt;

impl Default for MacroEnvironment {
    fn default() -> Self {
        MacroEnvironment::new()
    }
}
