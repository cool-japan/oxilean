//! # WatchError - Trait Implementations
//!
//! This module contains trait implementations for `WatchError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WatchError;
use std::fmt;

impl std::fmt::Display for WatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} (retries: {})",
            if self.recoverable {
                "recoverable"
            } else {
                "fatal"
            },
            self.message,
            self.retry_count
        )
    }
}
