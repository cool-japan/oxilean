//! # ElabConfig - Trait Implementations
//!
//! This module contains trait implementations for `ElabConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabConfig;

impl Default for ElabConfig {
    fn default() -> Self {
        Self {
            max_depth: 512,
            proof_irrelevance: true,
            auto_implicit: true,
            strict_instances: false,
            max_tactic_steps: 100_000,
            trace_elaboration: false,
            kernel_check: true,
            allow_sorry: false,
            max_universe_level: 100,
        }
    }
}
