//! # ConvConfig - Trait Implementations
//!
//! This module contains trait implementations for `ConvConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{DEFAULT_CONV_MAX_DEPTH, MAX_CONV_REWRITES};
use super::types::ConvConfig;

impl Default for ConvConfig {
    fn default() -> Self {
        Self {
            max_depth: DEFAULT_CONV_MAX_DEPTH,
            allow_simp: true,
            allow_ring: true,
            allow_norm_num: true,
            max_rewrites: MAX_CONV_REWRITES,
            auto_close: true,
        }
    }
}
