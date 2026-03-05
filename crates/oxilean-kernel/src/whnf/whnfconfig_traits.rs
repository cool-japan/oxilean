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
            beta: true,
            zeta: true,
            delta: true,
            iota: true,
            max_steps: 0,
        }
    }
}
