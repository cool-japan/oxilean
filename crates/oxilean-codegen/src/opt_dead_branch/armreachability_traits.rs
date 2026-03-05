//! # ArmReachability - Trait Implementations
//!
//! This module contains trait implementations for `ArmReachability`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ArmReachability;
use std::fmt;

impl std::fmt::Display for ArmReachability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmReachability::Reachable => write!(f, "Reachable"),
            ArmReachability::Unreachable => write!(f, "Unreachable"),
            ArmReachability::MaybeReachable => write!(f, "MaybeReachable"),
        }
    }
}
