//! # DartBackend - Trait Implementations
//!
//! This module contains trait implementations for `DartBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::DartBackend;

impl Default for DartBackend {
    fn default() -> Self {
        DartBackend::new()
    }
}
