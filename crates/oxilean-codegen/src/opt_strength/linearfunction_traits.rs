//! # LinearFunction - Trait Implementations
//!
//! This module contains trait implementations for `LinearFunction`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LinearFunction;
use std::fmt;

impl fmt::Display for LinearFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} * {:?} + {}", self.a, self.iv, self.b)
    }
}
