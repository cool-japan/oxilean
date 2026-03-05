//! # ReferenceContext - Trait Implementations
//!
//! This module contains trait implementations for `ReferenceContext`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ReferenceContext;
use std::fmt;

impl Default for ReferenceContext {
    fn default() -> Self {
        Self {
            include_declaration: true,
        }
    }
}
