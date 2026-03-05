//! # SRExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `SRExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::SRExtPassConfig;

impl Default for SRExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
