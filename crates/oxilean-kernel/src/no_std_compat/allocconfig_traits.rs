//! # AllocConfig - Trait Implementations
//!
//! This module contains trait implementations for `AllocConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AllocConfig;

impl Default for AllocConfig {
    fn default() -> Self {
        Self {
            use_global_allocator: true,
            max_heap_bytes: None,
        }
    }
}
