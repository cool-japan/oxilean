//! # PlatformCondition - Trait Implementations
//!
//! This module contains trait implementations for `PlatformCondition`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PlatformCondition;
use std::fmt;

impl std::fmt::Display for PlatformCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Os(s) => write!(f, "os={}", s),
            Self::Arch(s) => write!(f, "arch={}", s),
            Self::Triple(s) => write!(f, "triple={}", s),
            Self::And(conds) => {
                let parts: Vec<_> = conds.iter().map(|c| format!("{}", c)).collect();
                write!(f, "({})", parts.join(" AND "))
            }
            Self::Or(conds) => {
                let parts: Vec<_> = conds.iter().map(|c| format!("{}", c)).collect();
                write!(f, "({})", parts.join(" OR "))
            }
            Self::Not(c) => write!(f, "NOT({})", c),
            Self::Any => write!(f, "any"),
        }
    }
}
