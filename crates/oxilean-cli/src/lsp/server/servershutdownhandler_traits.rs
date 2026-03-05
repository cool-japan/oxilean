//! # ServerShutdownHandler - Trait Implementations
//!
//! This module contains trait implementations for `ServerShutdownHandler`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ServerShutdownHandler;
use std::fmt;

impl Default for ServerShutdownHandler {
    fn default() -> Self {
        Self::new()
    }
}
