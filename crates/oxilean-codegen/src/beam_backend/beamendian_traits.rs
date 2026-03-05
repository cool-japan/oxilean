//! # BeamEndian - Trait Implementations
//!
//! This module contains trait implementations for `BeamEndian`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::BeamEndian;
use std::fmt;

impl fmt::Display for BeamEndian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeamEndian::Big => write!(f, "big"),
            BeamEndian::Little => write!(f, "little"),
            BeamEndian::Native => write!(f, "native"),
        }
    }
}
