//! # JsModule - Trait Implementations
//!
//! This module contains trait implementations for `JsModule`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JsModule;

impl Default for JsModule {
    fn default() -> Self {
        Self::new()
    }
}
