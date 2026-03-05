//! # JvmConfig - Trait Implementations
//!
//! This module contains trait implementations for `JvmConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JvmConfig;

impl Default for JvmConfig {
    fn default() -> Self {
        JvmConfig {
            package: "oxilean.generated".to_string(),
            class_version: 65,
            emit_line_numbers: false,
            sealed_adt: true,
        }
    }
}
