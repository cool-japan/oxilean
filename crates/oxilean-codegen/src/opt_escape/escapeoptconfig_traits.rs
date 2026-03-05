//! # EscapeOptConfig - Trait Implementations
//!
//! This module contains trait implementations for `EscapeOptConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EscapeOptConfig;

impl Default for EscapeOptConfig {
    fn default() -> Self {
        EscapeOptConfig {
            enable_stack_alloc: true,
            max_stack_size_bytes: 512,
            aggressive_mode: false,
        }
    }
}
