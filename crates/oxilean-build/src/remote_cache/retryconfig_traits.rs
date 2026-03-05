//! # RetryConfig - Trait Implementations
//!
//! This module contains trait implementations for `RetryConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RetryConfig;

impl Default for RetryConfig {
    fn default() -> Self {
        Self::default_remote()
    }
}
