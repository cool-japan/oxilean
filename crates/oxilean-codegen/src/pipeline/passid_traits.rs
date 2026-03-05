//! # PassId - Trait Implementations
//!
//! This module contains trait implementations for `PassId`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::PassId;
use std::fmt;

impl fmt::Display for PassId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PassId::JoinPoints => write!(f, "join-points"),
            PassId::Specialize => write!(f, "specialize"),
            PassId::Reuse => write!(f, "reuse"),
            PassId::Dce => write!(f, "dce"),
            PassId::ClosureConvert => write!(f, "closure-convert"),
            PassId::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}
