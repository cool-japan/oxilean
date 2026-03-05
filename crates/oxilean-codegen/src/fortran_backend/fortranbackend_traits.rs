//! # FortranBackend - Trait Implementations
//!
//! This module contains trait implementations for `FortranBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FortranBackend;

impl Default for FortranBackend {
    fn default() -> Self {
        FortranBackend::new()
    }
}
