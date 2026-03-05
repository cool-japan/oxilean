//! # CSEPass - Trait Implementations
//!
//! This module contains trait implementations for `CSEPass`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{CSEPass, CseConfig};

impl Default for CSEPass {
    fn default() -> Self {
        CSEPass::new(CseConfig::default())
    }
}
