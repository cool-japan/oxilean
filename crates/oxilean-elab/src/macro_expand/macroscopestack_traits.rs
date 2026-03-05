//! # MacroScopeStack - Trait Implementations
//!
//! This module contains trait implementations for `MacroScopeStack`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MacroScopeStack;
use std::fmt;

impl Default for MacroScopeStack {
    fn default() -> Self {
        MacroScopeStack::new()
    }
}
