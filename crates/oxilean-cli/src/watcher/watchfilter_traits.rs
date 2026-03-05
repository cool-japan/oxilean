//! # WatchFilter - Trait Implementations
//!
//! This module contains trait implementations for `WatchFilter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WatchFilter;
use std::fmt;

impl Default for WatchFilter {
    fn default() -> Self {
        Self {
            extensions: vec!["lean".to_string(), "olean".to_string()],
            ignore_patterns: vec![
                ".git".to_string(),
                "build".to_string(),
                "node_modules".to_string(),
            ],
            include_hidden: false,
        }
    }
}
