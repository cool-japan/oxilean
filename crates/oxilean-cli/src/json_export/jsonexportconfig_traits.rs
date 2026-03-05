//! # JsonExportConfig - Trait Implementations
//!
//! This module contains trait implementations for `JsonExportConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::JsonExportConfig;
use std::fmt;

impl Default for JsonExportConfig {
    fn default() -> Self {
        Self {
            include_spans: true,
            include_types: true,
            pretty: true,
            max_depth: 100,
            include_proofs: true,
        }
    }
}
