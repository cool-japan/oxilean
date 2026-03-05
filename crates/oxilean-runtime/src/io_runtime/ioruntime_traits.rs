//! # IoRuntime - Trait Implementations
//!
//! This module contains trait implementations for `IoRuntime`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IoRuntime;
use std::fmt;

impl Default for IoRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for IoRuntime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IoRuntime")
            .field("refs", &self.refs.len())
            .field("io_enabled", &self.io_enabled)
            .field("has_output_buffer", &self.output_buffer.is_some())
            .field("input_queue_len", &self.input_queue.len())
            .finish()
    }
}
