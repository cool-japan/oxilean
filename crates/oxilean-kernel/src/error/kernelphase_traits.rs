//! # KernelPhase - Trait Implementations
//!
//! This module contains trait implementations for `KernelPhase`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::KernelPhase;
use std::fmt;

impl fmt::Display for KernelPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelPhase::Parse => write!(f, "parse"),
            KernelPhase::Elab => write!(f, "elab"),
            KernelPhase::TypeCheck => write!(f, "type-check"),
            KernelPhase::Reduction => write!(f, "reduction"),
        }
    }
}
