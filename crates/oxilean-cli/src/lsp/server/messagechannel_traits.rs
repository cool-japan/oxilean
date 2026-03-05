//! # MessageChannel - Trait Implementations
//!
//! This module contains trait implementations for `MessageChannel`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MessageChannel;
use std::fmt;

impl Default for MessageChannel {
    fn default() -> Self {
        Self::new()
    }
}
