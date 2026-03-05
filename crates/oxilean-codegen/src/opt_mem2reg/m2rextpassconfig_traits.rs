//! # M2RExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `M2RExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::M2RExtPassConfig;

impl Default for M2RExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
