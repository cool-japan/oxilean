//! # CtfeConfig - Trait Implementations
//!
//! This module contains trait implementations for `CtfeConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeConfig;

impl Default for CtfeConfig {
    fn default() -> Self {
        CtfeConfig {
            fuel: 10_000,
            max_depth: 256,
            replace_calls: true,
            cross_boundary_propagation: true,
        }
    }
}
