//! # JavaBackend - Trait Implementations
//!
//! This module contains trait implementations for `JavaBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JavaBackend;

impl Default for JavaBackend {
    fn default() -> Self {
        JavaBackend::new()
    }
}
