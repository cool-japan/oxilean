//! # ServerHealthChecker - Trait Implementations
//!
//! This module contains trait implementations for `ServerHealthChecker`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ServerHealthChecker;
use std::fmt;

impl Default for ServerHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}
