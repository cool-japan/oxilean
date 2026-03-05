//! # LoopTransformConfig - Trait Implementations
//!
//! This module contains trait implementations for `LoopTransformConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LoopTransformConfig;

impl Default for LoopTransformConfig {
    fn default() -> Self {
        LoopTransformConfig {
            unroll_factor: 4,
            tile_size: 64,
            interchange: false,
            strip_mine: true,
        }
    }
}
