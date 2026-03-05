//! # SwiftBackend - Trait Implementations
//!
//! This module contains trait implementations for `SwiftBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::SwiftBackend;

impl Default for SwiftBackend {
    fn default() -> Self {
        Self::new()
    }
}
