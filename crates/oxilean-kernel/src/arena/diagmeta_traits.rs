//! # DiagMeta - Trait Implementations
//!
//! This module contains trait implementations for `DiagMeta`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiagMeta;

impl std::fmt::Display for DiagMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.entries
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Default for DiagMeta {
    fn default() -> Self {
        Self::new()
    }
}
