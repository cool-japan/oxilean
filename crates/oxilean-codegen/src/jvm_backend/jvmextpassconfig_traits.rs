//! # JVMExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `JVMExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JVMExtPassConfig;

impl Default for JVMExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
