//! # AdvDeriveRegistry - Trait Implementations
//!
//! This module contains trait implementations for `AdvDeriveRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AdvDeriveRegistry;
use std::fmt;

impl Default for AdvDeriveRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}
