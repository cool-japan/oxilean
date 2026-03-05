//! # RcPolicy - Trait Implementations
//!
//! This module contains trait implementations for `RcPolicy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RcPolicy;
use std::fmt;

impl fmt::Display for RcPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RcPolicy::Standard => write!(f, "standard"),
            RcPolicy::Deferred => write!(f, "deferred"),
            RcPolicy::AggressiveElision => write!(f, "aggressive-elision"),
            RcPolicy::Disabled => write!(f, "disabled"),
        }
    }
}
