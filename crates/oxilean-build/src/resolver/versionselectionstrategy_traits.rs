//! # VersionSelectionStrategy - Trait Implementations
//!
//! This module contains trait implementations for `VersionSelectionStrategy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VersionSelectionStrategy;
use std::fmt;

impl fmt::Display for VersionSelectionStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Maximal => write!(f, "maximal"),
            Self::Minimal => write!(f, "minimal"),
            Self::Locked => write!(f, "locked"),
            Self::Pinned(v) => write!(f, "pinned({})", v),
        }
    }
}
