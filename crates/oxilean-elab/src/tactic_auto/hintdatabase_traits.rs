//! # HintDatabase - Trait Implementations
//!
//! This module contains trait implementations for `HintDatabase`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HintDatabase;
use std::fmt;

impl Default for HintDatabase {
    fn default() -> Self {
        Self::new()
    }
}
