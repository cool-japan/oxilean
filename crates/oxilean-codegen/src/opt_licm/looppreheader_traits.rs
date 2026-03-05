//! # LoopPreheader - Trait Implementations
//!
//! This module contains trait implementations for `LoopPreheader`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LoopPreheader;
use std::fmt;

impl std::fmt::Display for LoopPreheader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Preheader({} -> block#{})",
            self.loop_id, self.preheader_block
        )
    }
}
