//! # PipeExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `PipeExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::PipeExtPassConfig;

impl Default for PipeExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
