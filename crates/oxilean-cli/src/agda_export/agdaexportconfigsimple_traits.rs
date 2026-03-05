//! # AgdaExportConfigSimple - Trait Implementations
//!
//! This module contains trait implementations for `AgdaExportConfigSimple`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AgdaExportConfigSimple;
use std::fmt;

#[allow(dead_code)]
impl Default for AgdaExportConfigSimple {
    fn default() -> Self {
        Self {
            module_prefix: String::new(),
            use_unicode: true,
            emit_pragmas: true,
            indent_size: 2,
        }
    }
}
