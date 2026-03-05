//! # BvDecideConfig - Trait Implementations
//!
//! This module contains trait implementations for `BvDecideConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BvDecideConfig;

impl Default for BvDecideConfig {
    fn default() -> Self {
        BvDecideConfig {
            max_vars: 100_000,
            max_clauses: 1_000_000,
            timeout_ms: 30_000,
            preprocessing: true,
            enable_cdcl: true,
            vsids_decay: 0.95,
            restart_limit: 100,
            verbose: false,
        }
    }
}
