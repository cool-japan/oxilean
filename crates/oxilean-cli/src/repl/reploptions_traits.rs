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
use std::fmt;

impl Default for ReplOptions {
    fn default() -> Self {
        Self {
            pp_all: false,
            pp_implicit: false,
            pp_universes: false,
            pp_unicode: true,
            pp_width: 100,
            show_timing: false,
            auto_complete: true,
        }
    }
}
