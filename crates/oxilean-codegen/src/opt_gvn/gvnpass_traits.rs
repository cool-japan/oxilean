//! # GVNPass - Trait Implementations
//!
//! This module contains trait implementations for `GVNPass`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{GVNConfig, GVNPass};

impl Default for GVNPass {
    fn default() -> Self {
        Self::new(GVNConfig::default())
    }
}
