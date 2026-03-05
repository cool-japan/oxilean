//! # CommandRegistry - Trait Implementations
//!
//! This module contains trait implementations for `CommandRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CommandRegistry;
use std::fmt;

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}
