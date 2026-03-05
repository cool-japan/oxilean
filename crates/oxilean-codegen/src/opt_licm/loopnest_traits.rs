//! # LoopNest - Trait Implementations
//!
//! This module contains trait implementations for `LoopNest`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LoopNest;
use std::fmt;

impl std::fmt::Display for LoopNest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ids: Vec<String> = self.loops.iter().map(|l| l.to_string()).collect();
        write!(
            f,
            "LoopNest({})[perfect={}]",
            ids.join("->"),
            self.is_perfect
        )
    }
}
