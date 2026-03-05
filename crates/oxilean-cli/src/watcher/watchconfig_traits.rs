//! # WatchConfig - Trait Implementations
//!
//! This module contains trait implementations for `WatchConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{WatchAction, WatchConfig};
use std::fmt;

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            debounce_ms: 200,
            recursive: true,
            extensions: vec!["lean".to_string()],
            ignore_dirs: vec![".git".to_string(), "build".to_string()],
            action: WatchAction::Recheck,
        }
    }
}
