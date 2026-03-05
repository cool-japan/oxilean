//! # JsBackendConfig - Trait Implementations
//!
//! This module contains trait implementations for `JsBackendConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{JsBackendConfig, JsModuleFormat};
use std::fmt;

impl Default for JsBackendConfig {
    fn default() -> Self {
        JsBackendConfig {
            use_bigint_for_nat: true,
            strict_mode: false,
            include_runtime: true,
            emit_jsdoc: false,
            module_format: JsModuleFormat::Es,
            minify: false,
        }
    }
}

impl fmt::Display for JsBackendConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JsBackendConfig {{ bigint={}, strict={}, runtime={}, minify={} }}",
            self.use_bigint_for_nat, self.strict_mode, self.include_runtime, self.minify,
        )
    }
}
