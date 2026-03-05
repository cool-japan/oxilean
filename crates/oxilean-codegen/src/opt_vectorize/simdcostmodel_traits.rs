//! # SIMDCostModel - Trait Implementations
//!
//! This module contains trait implementations for `SIMDCostModel`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SIMDCostModel;

impl Default for SIMDCostModel {
    fn default() -> Self {
        SIMDCostModel {
            scalar_ipc: 2.0,
            vector_ipc: 1.5,
            mem_bandwidth_bpc: 32.0,
        }
    }
}
