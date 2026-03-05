//! # MessageBatch - Trait Implementations
//!
//! This module contains trait implementations for `MessageBatch`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MessageBatch;
use std::fmt;

impl Default for MessageBatch {
    fn default() -> Self {
        Self::new()
    }
}
