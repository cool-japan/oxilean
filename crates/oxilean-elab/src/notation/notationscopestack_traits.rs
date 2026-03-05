//! # NotationScopeStack - Trait Implementations
//!
//! This module contains trait implementations for `NotationScopeStack`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NotationScopeStack;
use std::fmt;

impl Default for NotationScopeStack {
    fn default() -> Self {
        NotationScopeStack::new()
    }
}
