//! # CastDirection - Trait Implementations
//!
//! This module contains trait implementations for `CastDirection`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CastDirection;
use std::fmt;

impl fmt::Display for CastDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastDirection::Push => write!(f, "push"),
            CastDirection::Pull => write!(f, "pull"),
            CastDirection::Squash => write!(f, "squash"),
        }
    }
}
