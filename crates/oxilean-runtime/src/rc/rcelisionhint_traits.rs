//! # RcElisionHint - Trait Implementations
//!
//! This module contains trait implementations for `RcElisionHint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RcElisionHint;
use std::fmt;

impl fmt::Display for RcElisionHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RcElisionHint::None => write!(f, "none"),
            RcElisionHint::LinearUse => write!(f, "linear"),
            RcElisionHint::Ephemeral => write!(f, "ephemeral"),
            RcElisionHint::Borrowed => write!(f, "borrowed"),
            RcElisionHint::UniqueOwner => write!(f, "unique"),
            RcElisionHint::SharedImmutable => write!(f, "shared-imm"),
            RcElisionHint::TailPosition => write!(f, "tail"),
            RcElisionHint::UncapturedArg => write!(f, "uncaptured"),
            RcElisionHint::StructField => write!(f, "field"),
            RcElisionHint::DeadPath => write!(f, "dead"),
        }
    }
}
