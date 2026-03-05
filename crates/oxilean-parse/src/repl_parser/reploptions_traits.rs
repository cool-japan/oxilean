//! # ReplOptions - Trait Implementations
//!
//! This module contains trait implementations for `ReplOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ReplOptions;

impl Default for ReplOptions {
    fn default() -> Self {
        Self {
            show_timing: false,
            print_types: true,
            max_history: 100,
            color: true,
            verbose: false,
        }
    }
}
