//! # AesopRuleSafety - Trait Implementations
//!
//! This module contains trait implementations for `AesopRuleSafety`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AesopRuleSafety;

impl std::fmt::Display for AesopRuleSafety {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AesopRuleSafety::Safe => write!(f, "safe"),
            AesopRuleSafety::AlmostSafe => write!(f, "almost_safe"),
            AesopRuleSafety::Unsafe => write!(f, "unsafe"),
        }
    }
}
