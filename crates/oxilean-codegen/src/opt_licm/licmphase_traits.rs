//! # LICMPhase - Trait Implementations
//!
//! This module contains trait implementations for `LICMPhase`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LICMPhase;
use std::fmt;

impl std::fmt::Display for LICMPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LICMPhase::LICMBeforeCSE => write!(f, "LICM-before-CSE"),
            LICMPhase::LICMAfterCSE => write!(f, "LICM-after-CSE"),
            LICMPhase::LICMIterative => write!(f, "LICM-iterative"),
            LICMPhase::LICMOnce => write!(f, "LICM-once"),
        }
    }
}
