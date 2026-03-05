//! # IdrisCodegenConfig - Trait Implementations
//!
//! This module contains trait implementations for `IdrisCodegenConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{IdrisCodegenConfig, Totality};

impl Default for IdrisCodegenConfig {
    fn default() -> Self {
        IdrisCodegenConfig {
            emit_docs: true,
            emit_logging: false,
            default_totality: Totality::Total,
            auto_inline: false,
            auto_inline_limit: 5,
            emit_name_pragmas: true,
            emit_header_comment: true,
            target_backend: "chez".to_string(),
        }
    }
}
