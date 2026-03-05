//! # BetaEtaPass - Trait Implementations
//!
//! This module contains trait implementations for `BetaEtaPass`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{BetaEtaConfig, BetaEtaPass};

impl Default for BetaEtaPass {
    fn default() -> Self {
        BetaEtaPass::new(BetaEtaConfig::default())
    }
}
