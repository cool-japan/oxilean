//! # GoBackend - Trait Implementations
//!
//! This module contains trait implementations for `GoBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::GoBackend;

impl Default for GoBackend {
    fn default() -> Self {
        Self::new()
    }
}
