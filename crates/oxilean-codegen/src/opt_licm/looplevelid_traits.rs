//! # LoopLevelId - Trait Implementations
//!
//! This module contains trait implementations for `LoopLevelId`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LoopLevelId;
use std::fmt;

impl std::fmt::Display for LoopLevelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "loop#{}", self.0)
    }
}
