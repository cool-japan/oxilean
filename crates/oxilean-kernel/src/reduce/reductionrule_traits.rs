//! # ReductionRule - Trait Implementations
//!
//! This module contains trait implementations for `ReductionRule`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ReductionRule;

impl std::fmt::Display for ReductionRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReductionRule::Beta => write!(f, "beta"),
            ReductionRule::Delta => write!(f, "delta"),
            ReductionRule::Zeta => write!(f, "zeta"),
            ReductionRule::Iota => write!(f, "iota"),
            ReductionRule::Proj => write!(f, "proj"),
            ReductionRule::Quot => write!(f, "quot"),
            ReductionRule::NatLit => write!(f, "nat_lit"),
            ReductionRule::StrLit => write!(f, "str_lit"),
            ReductionRule::None => write!(f, "none"),
        }
    }
}
