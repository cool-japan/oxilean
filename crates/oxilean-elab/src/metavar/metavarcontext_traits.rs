//! # MetaVarContext - Trait Implementations
//!
//! This module contains trait implementations for `MetaVarContext`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metavarcontext_type::MetaVarContext;
use std::fmt;

impl Default for MetaVarContext {
    fn default() -> Self {
        Self::new()
    }
}
