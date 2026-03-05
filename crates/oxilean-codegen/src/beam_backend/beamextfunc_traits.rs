//! # BeamExtFunc - Trait Implementations
//!
//! This module contains trait implementations for `BeamExtFunc`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::BeamExtFunc;
use std::fmt;

impl fmt::Display for BeamExtFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}/{}", self.module, self.function, self.arity)
    }
}
