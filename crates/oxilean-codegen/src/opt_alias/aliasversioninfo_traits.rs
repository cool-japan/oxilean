//! # AliasVersionInfo - Trait Implementations
//!
//! This module contains trait implementations for `AliasVersionInfo`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AliasVersionInfo;
use std::fmt;

impl Default for AliasVersionInfo {
    fn default() -> Self {
        Self {
            pass_version: 2,
            supports_tbaa: true,
            supports_field_sensitivity: false,
            supports_context_sensitivity: false,
            default_level: "andersen".to_string(),
        }
    }
}

impl std::fmt::Display for AliasVersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AliasVersionInfo {{ v={}, tbaa={}, field={}, ctx={}, level={} }}",
            self.pass_version,
            self.supports_tbaa,
            self.supports_field_sensitivity,
            self.supports_context_sensitivity,
            self.default_level,
        )
    }
}
