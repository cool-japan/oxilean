//! # WhnfConfig - Trait Implementations
//!
//! This module contains trait implementations for `WhnfConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WhnfConfig;

impl Default for WhnfConfig {
    fn default() -> Self {
        Self {
            max_beta_steps: 1024,
            unfold_let: true,
            reduce_iota: true,
            reduce_zeta: true,
        }
    }
}
