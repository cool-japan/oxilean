//! # CCExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `CCExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CCExtPassConfig;

impl Default for CCExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
