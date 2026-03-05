//! # ReuseConfigExt - Trait Implementations
//!
//! This module contains trait implementations for `ReuseConfigExt`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseConfigExt;

impl Default for ReuseConfigExt {
    fn default() -> Self {
        Self {
            enable_reuse: true,
            enable_stack_alloc: true,
            enable_inline: true,
            enable_scratch_buffer: false,
            max_reuse_distance: 1000,
            max_live_ranges: 10_000,
            scratch_buffer_size: 65536,
        }
    }
}
