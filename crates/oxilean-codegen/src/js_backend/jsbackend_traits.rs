//! # JsBackend - Trait Implementations
//!
//! This module contains trait implementations for `JsBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JsBackend;

impl Default for JsBackend {
    fn default() -> Self {
        Self::new()
    }
}
