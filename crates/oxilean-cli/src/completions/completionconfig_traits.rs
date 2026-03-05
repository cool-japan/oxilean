//! # CompletionConfig - Trait Implementations
//!
//! This module contains trait implementations for `CompletionConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{CompletionConfig, CompletionOutputFormat};
use std::fmt;

impl Default for CompletionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_ttl_ms: 5000,
            max_candidates: 50,
            fuzzy_matching: true,
            history_enabled: true,
            history_max_entries: 1000,
            output_format: CompletionOutputFormat::Lines,
        }
    }
}
