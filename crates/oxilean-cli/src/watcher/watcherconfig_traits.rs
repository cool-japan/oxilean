//! # WatcherConfig - Trait Implementations
//!
//! This module contains trait implementations for `WatcherConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WatcherConfig;
use std::fmt;

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 500,
            debounce_ms: 200,
            max_depth: 10,
            include_extensions: vec!["lean".to_string()],
            exclude_patterns: vec![
                ".git".to_string(),
                "target".to_string(),
                ".oxilean_cache".to_string(),
            ],
            follow_symlinks: false,
            batch_events: true,
            batch_window_ms: 100,
        }
    }
}
