//! # BetaEtaConfig - Trait Implementations
//!
//! This module contains trait implementations for `BetaEtaConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BetaEtaConfig;

impl Default for BetaEtaConfig {
    fn default() -> Self {
        BetaEtaConfig {
            max_depth: 256,
            do_eta: true,
            do_beta: true,
        }
    }
}
