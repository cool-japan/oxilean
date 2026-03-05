//! # EvmOptPass - Trait Implementations
//!
//! This module contains trait implementations for `EvmOptPass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmOptPass;
use std::fmt;

impl std::fmt::Display for EvmOptPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvmOptPass::DeadCodeElim => write!(f, "dce"),
            EvmOptPass::ConstantFolding => write!(f, "const_fold"),
            EvmOptPass::CommonSubexprElim => write!(f, "cse"),
            EvmOptPass::InlineFunctions => write!(f, "inline"),
            EvmOptPass::JumpElim => write!(f, "jump_elim"),
            EvmOptPass::PushPop => write!(f, "push_pop"),
            EvmOptPass::Peephole => write!(f, "peephole"),
        }
    }
}
