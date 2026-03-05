//! # ReuseDecision - Trait Implementations
//!
//! This module contains trait implementations for `ReuseDecision`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseDecision;
use std::fmt;

impl std::fmt::Display for ReuseDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReuseDecision::Reuse(id) => write!(f, "reuse#{}", id),
            ReuseDecision::NewAlloc => write!(f, "new_alloc"),
            ReuseDecision::RcBump(id) => write!(f, "rc_bump#{}", id),
            ReuseDecision::StackAlloc => write!(f, "stack_alloc"),
            ReuseDecision::Inline => write!(f, "inline"),
            ReuseDecision::ScratchBuffer(id) => write!(f, "scratch#{}", id),
        }
    }
}
