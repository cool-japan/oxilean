//! # PoolConfig - Trait Implementations
//!
//! This module contains trait implementations for `PoolConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PoolConfig;

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            block_size: 64,
            max_blocks: 0,
            growth_factor: 2.0,
        }
    }
}
