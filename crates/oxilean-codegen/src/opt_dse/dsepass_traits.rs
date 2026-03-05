//! # DSEPass - Trait Implementations
//!
//! This module contains trait implementations for `DSEPass`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{DSEPass, DeadStoreConfig};

impl Default for DSEPass {
    fn default() -> Self {
        Self::new(DeadStoreConfig::default())
    }
}
