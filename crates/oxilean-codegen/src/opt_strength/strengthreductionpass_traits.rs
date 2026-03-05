//! # StrengthReductionPass - Trait Implementations
//!
//! This module contains trait implementations for `StrengthReductionPass`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{StrengthConfig, StrengthReductionPass};

impl Default for StrengthReductionPass {
    fn default() -> Self {
        StrengthReductionPass::new(StrengthConfig::default())
    }
}
