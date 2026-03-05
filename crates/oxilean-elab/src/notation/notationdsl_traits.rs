//! # NotationDSL - Trait Implementations
//!
//! This module contains trait implementations for `NotationDSL`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NotationDSL;
use std::fmt;

impl Default for NotationDSL {
    fn default() -> Self {
        NotationDSL::new()
    }
}
