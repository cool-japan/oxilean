//! # DeadBranchOptKind - Trait Implementations
//!
//! This module contains trait implementations for `DeadBranchOptKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DeadBranchOptKind;
use std::fmt;

impl std::fmt::Display for DeadBranchOptKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeadBranchOptKind::ArmEliminated => write!(f, "ArmEliminated"),
            DeadBranchOptKind::CaseFolded => write!(f, "CaseFolded"),
            DeadBranchOptKind::UniformReturn => write!(f, "UniformReturn"),
            DeadBranchOptKind::SingleArmInlined => write!(f, "SingleArmInlined"),
            DeadBranchOptKind::UnreachableRemoved => write!(f, "UnreachableRemoved"),
        }
    }
}
