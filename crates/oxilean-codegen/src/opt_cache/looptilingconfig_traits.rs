//! # LoopTilingConfig - Trait Implementations
//!
//! This module contains trait implementations for `LoopTilingConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LoopTilingConfig;

impl Default for LoopTilingConfig {
    fn default() -> Self {
        LoopTilingConfig {
            tile_size_l1: 64,
            tile_size_l2: 128,
            enable_l1_tiling: true,
            enable_l2_tiling: true,
        }
    }
}
