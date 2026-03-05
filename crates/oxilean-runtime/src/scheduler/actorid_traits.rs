//! # ActorId - Trait Implementations
//!
//! This module contains trait implementations for `ActorId`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ActorId;
use std::fmt;

impl fmt::Display for ActorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "actor#{}", self.0)
    }
}
