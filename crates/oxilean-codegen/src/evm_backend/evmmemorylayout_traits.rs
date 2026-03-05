//! # EvmMemoryLayout - Trait Implementations
//!
//! This module contains trait implementations for `EvmMemoryLayout`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmMemoryLayout;

impl Default for EvmMemoryLayout {
    fn default() -> Self {
        Self {
            scratch_space: (0x00, 0x40),
            free_mem_ptr: 0x40,
            zero_slot: 0x60,
            initial_free: 0x80,
        }
    }
}
