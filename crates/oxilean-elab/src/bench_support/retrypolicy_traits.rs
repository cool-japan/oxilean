//! # RetryPolicy - Trait Implementations
//!
//! This module contains trait implementations for `RetryPolicy`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RetryPolicy;
use std::fmt;

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::lenient()
    }
}
