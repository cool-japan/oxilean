//! # GVNConfig - Trait Implementations
//!
//! This module contains trait implementations for `GVNConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::GVNConfig;

impl Default for GVNConfig {
    fn default() -> Self {
        GVNConfig {
            do_phi_translation: true,
            max_depth: 100,
        }
    }
}
