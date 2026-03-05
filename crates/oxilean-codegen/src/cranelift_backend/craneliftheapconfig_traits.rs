//! # CraneliftHeapConfig - Trait Implementations
//!
//! This module contains trait implementations for `CraneliftHeapConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CraneliftHeapConfig;

impl Default for CraneliftHeapConfig {
    fn default() -> Self {
        CraneliftHeapConfig {
            base: 0,
            min_size: 65536,
            max_size: None,
            page_size: 65536,
            needs_bounds_check: true,
            guard_size: 0,
        }
    }
}
