//! # DSEExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `DSEExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::DSEExtPassConfig;

impl Default for DSEExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
