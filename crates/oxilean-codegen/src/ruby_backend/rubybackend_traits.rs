//! # RubyBackend - Trait Implementations
//!
//! This module contains trait implementations for `RubyBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::RubyBackend;

impl Default for RubyBackend {
    fn default() -> Self {
        RubyBackend::new()
    }
}
