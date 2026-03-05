//! # CSEExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `CSEExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CSEExtPassConfig;

impl Default for CSEExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
