//! # CdclConfig - Trait Implementations
//!
//! This module contains trait implementations for `CdclConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CdclConfig;

impl Default for CdclConfig {
    fn default() -> Self {
        CdclConfig {
            max_conflicts: 1_000_000,
            restart_base: 100,
            gc_interval: 5000,
            gc_keep_fraction: 0.5,
            vsids_decay: 0.95,
        }
    }
}
